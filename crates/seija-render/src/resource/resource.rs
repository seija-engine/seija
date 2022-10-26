use std::{collections::HashMap, num::NonZeroU32, ops::Range, sync::Arc};
use bevy_ecs::world::World;
use seija_asset::{HandleId, Handle, AssetServer, Assets};
use seija_core::IDGenU64;
use wgpu::{BufferUsage, SwapChainError, TextureView, util::DeviceExt, TextureFormat};

use super::{Texture, TextureType};


pub const COPY_BYTES_PER_ROW_ALIGNMENT: usize = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;

#[derive(Debug,Clone,Hash,PartialEq, Eq)]
pub enum RenderResourceId {
    Buffer(BufferId),
    BufferAddr(BufferId,u64,u64),
    TextureView(TextureId),
    Sampler(SamplerId),
    MainSwap,

    Texture(Handle<Texture>)
}

impl RenderResourceId {
    pub fn into_texture_id(&self) -> Option<TextureId> {
        match self {
            RenderResourceId::TextureView(texture_id) => Some(*texture_id),
            _ => None
        }
    }

    pub fn into_sampler_id(&self) -> Option<SamplerId> {
        match self {
            RenderResourceId::Sampler(sampler_id) => Some(*sampler_id),
            _ => None
        }
    }

    pub fn into_texture(&self) -> Option<Handle<Texture>> {
        match self {
            RenderResourceId::Texture(texture) => Some(texture.clone_weak()),
            _ => None
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct BufferId(u64);

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TextureId(u64);

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct SamplerId(u64);


pub struct RenderResources {
    pub device: Arc<wgpu::Device>,
    main_surface:Option<wgpu::Surface>,
    main_swap_chain:Option<wgpu::SwapChain>,
    pub main_swap_chain_frame:Option<wgpu::SwapChainFrame>,

    pub default_textures:Vec<Handle<Texture>>,
   
    pub buffers: HashMap<BufferId, wgpu::Buffer>,
    pub textures: HashMap<TextureId, wgpu::Texture>,
    pub texture_views: HashMap<TextureId, wgpu::TextureView>,
    pub samplers: HashMap<SamplerId, wgpu::Sampler>,
    resources:HashMap<HandleId,[Option<RenderResourceId>;4]>,


    buffer_id_gen:IDGenU64,
    texture_id_gen:IDGenU64,
    sampler_id_gen:IDGenU64
}

impl RenderResources {
    pub fn new(device:Arc<wgpu::Device>,assets:&AssetServer) -> RenderResources {
       let h_white = assets.get_asset("texture:white").unwrap();
        RenderResources {
            device,
            main_surface:None,
            main_swap_chain:None,
            main_swap_chain_frame:None,
            buffers:HashMap::default(),
            textures:HashMap::default(),
            resources:HashMap::default(),
            buffer_id_gen:IDGenU64::new(),
            texture_id_gen:IDGenU64::new(),
            texture_views:HashMap::default(),
            samplers:HashMap::default(),
            sampler_id_gen:IDGenU64::new(),
            default_textures:vec![h_white.make_handle().typed()]
        }
    }

    pub fn set_main_surface(&mut self,surface:wgpu::Surface) {
        self.main_surface = Some(surface);
    }

    pub fn create_buffer(&mut self,desc:&wgpu::BufferDescriptor) -> BufferId {
        let buffer = self.device.create_buffer(desc);
       
        let id = BufferId(self.buffer_id_gen.next());
        self.buffers.insert(id, buffer);
        id
    }

    pub fn set_render_resource(&mut self,handle:&HandleId,res_id:RenderResourceId,idx:usize) {
        let entry = self.resources.entry(*handle).or_insert([None,None,None,None]);
        entry[idx] = Some(res_id);
    }

    pub fn get_render_resource(&self,handle:&HandleId,idx:usize) -> Option<&RenderResourceId> {
        self.resources.get(handle).and_then(|arr| arr[idx].as_ref())
    }

    pub fn remove_render_resource(&mut self,handle:&HandleId,idx:usize) {
        if let Some(arr) = self.resources.get_mut(handle) {
            arr[idx] = None;
            if arr.iter().all(|v| v.is_none()) {
                self.resources.remove(handle);
            }
        }
    }

    pub fn remove_buffer(&mut self,id:BufferId) {
        self.buffers.remove(&id);
    }

    pub fn create_buffer_with_data(&mut self,usage:BufferUsage,data:&[u8]) -> BufferId {
        let buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents:data,
            label:None,
            usage
        });
        let id = BufferId(self.buffer_id_gen.next());
        self.buffers.insert(id, buffer);
        id
    }

    pub fn map_buffer(&mut self,id:&BufferId,mode:wgpu::MapMode) {
        let buffer = self.buffers.get(id).unwrap();
        let buffer_slice = buffer.slice(..);
        let data = buffer_slice.map_async(mode);
        self.device.poll(wgpu::Maintain::Wait);
        if futures_lite::future::block_on(data).is_err() {
            panic!("Failed to map buffer to host.");
        }
    }

    pub fn unmap_buffer(&self, id: &BufferId) {
        let buffer =self.buffers.get(id).unwrap();
        buffer.unmap();
    }

    pub fn write_mapped_buffer(&self,id:&BufferId,range:Range<u64>,write:&mut dyn FnMut(&mut [u8],&RenderResources)) {
        let buffer = self.buffers.get(id).unwrap();
        let buffer_slice = buffer.slice(range);
        let mut data = buffer_slice.get_mapped_range_mut();
        write(&mut data, self);
    }
   

    pub fn create_swap_chain(&mut self,w:u32,h:u32,vsync:bool) {
        let desc = &wgpu::SwapChainDescriptor {
            usage:wgpu::TextureUsage::RENDER_ATTACHMENT,
            format:wgpu::TextureFormat::Bgra8Unorm,
            width:w,
            height:h,
            present_mode: if vsync {wgpu::PresentMode::Fifo} else {wgpu::PresentMode::Immediate}
        };
        let surface = self.main_surface.as_ref().unwrap();
        let swap_chain = self.device.create_swap_chain(surface, desc);
        self.main_swap_chain = Some(swap_chain);
        
    }

    pub fn next_swap_chain_texture(&mut self) -> Result<RenderResourceId,SwapChainError> {
       if self.main_swap_chain_frame.is_some() {
           return Ok(RenderResourceId::MainSwap)
       }
       self.main_swap_chain.as_mut().ok_or(SwapChainError::Lost)
                                    .and_then(|v| v.get_current_frame())
                                    .map(|s| {
                                        self.main_swap_chain_frame = Some(s);
                                        RenderResourceId::MainSwap
                                    })
    }

    pub fn get_texture_view_by_resid(&self,res_id:&RenderResourceId) -> Option<&TextureView> {
        match res_id {
            RenderResourceId::MainSwap => {
                
                self.main_swap_chain_frame.as_ref().map(|v| &v.output.view)
            }
            RenderResourceId::TextureView(texture_id) => {
                self.texture_views.get(texture_id)
            },
            RenderResourceId::Texture(h_tex) => {
                let view_res_id = self.get_render_resource(&h_tex.id, 0)?;
                self.get_texture_view_by_resid(view_res_id)
            },
            _ => None
        }
    }

    pub fn get_texture_view(&self,texture_id: &TextureId) -> Option<&wgpu::TextureView> {
        self.texture_views.get(texture_id)
    }

    pub fn get_sampler(&self,sampler_id: &SamplerId) -> Option<&wgpu::Sampler> {
        self.samplers.get(sampler_id)
    }

    pub fn get_buffer_by_resid(&self,res_id:&RenderResourceId) -> Option<&wgpu::Buffer> {
        match res_id {
            RenderResourceId::Buffer(buffer_id) => {
                Some( self.buffers.get(buffer_id).unwrap())
            }
            _ => None
        }  
    }

    pub fn get_texture_format(&mut self,resid:&RenderResourceId,world:&World) -> Option<TextureFormat> {
        match resid {
            RenderResourceId::MainSwap => {
               Some(wgpu::TextureFormat::Bgra8Unorm)
            },
            RenderResourceId::Texture(h_texture) => {
               let textures = world.get_resource::<Assets<Texture>>().unwrap();
               if let Some(texture) = textures.get(&h_texture.id) {
                 return Some(texture.desc().desc.format)
               }
               None
            }, 
            _ => None
        }
    }

    pub fn get_buffer(&self,buffer_id:&BufferId) -> Option<&wgpu::Buffer> {
        self.buffers.get(buffer_id)
    } 

    pub fn copy_buffer_to_buffer(
        &self,
        command_encoder: &mut wgpu::CommandEncoder,
        source_buffer: &BufferId,
        source_offset: u64,
        destination_buffer: &BufferId,
        destination_offset: u64,
        size: u64,
    ) {
        let source = self.buffers.get(source_buffer).unwrap();
        let destination = self.buffers.get(destination_buffer).unwrap();
        command_encoder.copy_buffer_to_buffer(
            source,
            source_offset,
            destination,
            destination_offset,
            size,
        );
    }

    pub fn remove_texture(&mut self,id:&RenderResourceId) {
        if let RenderResourceId::TextureView(tex_id) = id {
            self.textures.remove(tex_id);
            self.texture_views.remove(tex_id);
        }
    }

    pub fn create_texture(&mut self,texture_desc:&wgpu::TextureDescriptor,view_desc:&wgpu::TextureViewDescriptor) -> TextureId {
        let texture = self.device.create_texture(texture_desc);
        let texture_id = TextureId(self.texture_id_gen.next());
        
        let texture_view = texture.create_view(view_desc);

        self.textures.insert(texture_id, texture);
        self.texture_views.insert(texture_id, texture_view);
        
       
        texture_id
    }

    pub fn create_sampler(&mut self, sampler_desc: &wgpu::SamplerDescriptor) -> SamplerId {
        let sampler = self.device.create_sampler(sampler_desc);
        let sampler_id = SamplerId(self.sampler_id_gen.next());
        self.samplers.insert(sampler_id, sampler);
        sampler_id
    }

    pub fn fill_texture(&mut self,texture:&Texture,texture_id:&TextureId,command:&mut wgpu::CommandEncoder) {
        if let TextureType::Image(image_info) = &texture.texture {
            let desc = &texture.desc().desc;
            let width = desc.size.width as usize;
            let aligned_width = Self::get_aligned_texture_size(width);
            let format_size:usize = desc.format.describe().block_size as usize;
           
            let mut aligned_data = vec![0;format_size * 
                                                  aligned_width * 
                                                  desc.size.height as usize * 
                                                  desc.size.depth_or_array_layers as usize];
    
            image_info.data.chunks_exact(format_size * width)
                        .enumerate()
                        .for_each(|(index, row)| {
                                    let offset = index * aligned_width * format_size;
                                    aligned_data[offset..(offset + width * format_size)]
                                        .copy_from_slice(row);
                                  });
    
            let texture_buffer = self.create_buffer_with_data(wgpu::BufferUsage::COPY_SRC,&aligned_data);
            self.copy_buffer_to_texture(command, 
                           texture_buffer, 
                           0, 
                     NonZeroU32::new((format_size * aligned_width) as u32).unwrap(), 
                           texture_id,
                            wgpu::Origin3d::default(),
                         0, desc.size,
                        if desc.size.depth_or_array_layers > 1 { Some(NonZeroU32::new(desc.size.height).unwrap()) } else { None })
        }
        
    }

    pub fn is_texture_ready(&self,texture:&Handle<Texture>) -> bool {
        self.get_render_resource(&texture.id, 0).is_some()
    }

    pub fn is_textures_ready(&self,textures:&Vec<Handle<Texture>>) -> bool {
        for texture in textures.iter() {
            if !self.is_texture_ready(texture) {
                return false;
            }
        }
        true
    }

    pub fn copy_buffer_to_texture(&self,
                                  command: &mut wgpu::CommandEncoder,
                                  source_buffer: BufferId,
                                  source_offset: u64,
                                  source_bytes_per_row: NonZeroU32,
                                  dest_texture: &TextureId,
                                  dest_origin:wgpu::Origin3d,
                                  dest_mip_level: u32,
                                  size: wgpu::Extent3d,
                                  rows_per_image:Option<NonZeroU32>) {
        let source = self.buffers.get(&source_buffer).unwrap();
        let dest = self.textures.get(&dest_texture).unwrap();
        command.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer { 
                buffer: source, 
                layout: wgpu::ImageDataLayout { 
                    offset: source_offset, 
                    bytes_per_row: Some(source_bytes_per_row), 
                    rows_per_image
                }
            },wgpu::ImageCopyTexture { 
                texture: dest, 
                mip_level: dest_mip_level, 
                origin: dest_origin 
            },size);                          
    }

    fn get_aligned_texture_size(size: usize) -> usize {
        (size + COPY_BYTES_PER_ROW_ALIGNMENT - 1) & !(COPY_BYTES_PER_ROW_ALIGNMENT - 1)
    }

    pub fn clear_swap_chain_texture(&mut self) {
        self.main_swap_chain_frame = None;
    }

    pub fn is_ready(&self,res_id:&RenderResourceId) -> bool {
        match res_id {
            RenderResourceId::Texture(tex) => {
                !self.get_render_resource(&tex.id, 0).is_none()
            },
            _ => true
        }
    }
}