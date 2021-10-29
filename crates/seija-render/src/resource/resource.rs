use std::{collections::HashMap, ops::Range, sync::Arc};
use seija_asset::HandleUntyped;
use uuid::Uuid;
use wgpu::{Buffer, BufferUsage, Device, SwapChainError, TextureView, util::DeviceExt};

#[derive(Debug,Clone,Hash,PartialEq, Eq)]
pub enum RenderResourceId {
    Buffer(BufferId),
    BufferAddr(BufferId,u64,u64),
    MainSwap
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct BufferId(pub Uuid);

impl BufferId {
    pub fn new() -> BufferId { BufferId(Uuid::new_v4()) }
}


pub struct RenderResources {
    pub device: Arc<wgpu::Device>,
    main_surface:Option<wgpu::Surface>,
    main_swap_chain:Option<wgpu::SwapChain>,
    pub main_swap_chain_frame:Option<wgpu::SwapChainFrame>,

    pub buffers: HashMap<BufferId, wgpu::Buffer>,
    

    resources:HashMap<(HandleUntyped,u8),RenderResourceId>
}

impl RenderResources {
    pub fn new(device:Arc<wgpu::Device>) -> RenderResources {
        RenderResources {
            device,
            main_surface:None,
            main_swap_chain:None,
            main_swap_chain_frame:None,
            buffers:HashMap::default(),
            resources:HashMap::default()
        }
    }

    pub fn set_main_surface(&mut self,surface:wgpu::Surface) {
        self.main_surface = Some(surface);
    }

    pub fn create_buffer(&mut self,desc:&wgpu::BufferDescriptor) -> BufferId {
        let buffer = self.device.create_buffer(desc);
       
        let id = BufferId::new();
        self.buffers.insert(id, buffer);
        id
    }

    pub fn set_render_resource(&mut self,handle:HandleUntyped,res_id:RenderResourceId,idx:u8) {
        self.resources.insert((handle,idx), res_id);
    }

    pub fn get_render_resource(&self,handle:HandleUntyped,idx:u8) -> Option<RenderResourceId> {
        self.resources.get(&(handle,idx)).cloned()
    }

    pub fn remove_render_resource(&mut self,handle:HandleUntyped,idx:u8) {
        self.resources.remove(&(handle, idx));
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
        let id = BufferId::new();
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
            format:wgpu::TextureFormat::Bgra8UnormSrgb,
            width:w,
            height:h,
            present_mode: if vsync {wgpu::PresentMode::Fifo} else {wgpu::PresentMode::Immediate}
        };
        let surface = self.main_surface.as_ref().unwrap();
        let swap_chain = self.device.create_swap_chain(surface, desc);
        self.main_swap_chain = Some(swap_chain);
        
    }

    pub fn next_swap_chain_texture(&mut self) -> Result<RenderResourceId,SwapChainError> {
       self.main_swap_chain.as_mut().ok_or(SwapChainError::Lost)
                                    .and_then(|v| v.get_current_frame())
                                    .map(|s| {
                                        self.main_swap_chain_frame = Some(s);
                                        RenderResourceId::MainSwap
                                    })
    }

    pub fn get_texture_view(&self,res_id:&RenderResourceId) -> Option<&TextureView> {
        match res_id {
            RenderResourceId::MainSwap => {
                Some(&self.main_swap_chain_frame.as_ref().unwrap().output.view)
            }
            _ => None
        }
    }

    pub fn get_buffer(&self,res_id:&RenderResourceId) -> Option<&wgpu::Buffer> {
        match res_id {
            RenderResourceId::Buffer(buffer_id) => {
                Some( self.buffers.get(buffer_id).unwrap())
            }
            _ => None
        }  
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

    pub fn clear_swap_chain_texture(&mut self) {
        
        self.main_swap_chain_frame = None;
    }
}