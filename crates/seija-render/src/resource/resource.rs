use std::{collections::HashMap, ops::Range, sync::Arc};
use uuid::Uuid;

use crate::render::RenderContext;
#[derive(Debug,Clone,Hash,PartialEq, Eq)]
pub struct ResourceId(pub Uuid);

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct BufferId(pub Uuid);

impl BufferId {
    pub fn new() -> BufferId { BufferId(Uuid::new_v4()) }
}


pub struct RenderResources {
    pub device: Arc<wgpu::Device>,
    main_surface:Option<wgpu::Surface>,
    main_swap_chain:Option<wgpu::SwapChain>,
    main_swap_chain_frame:Option<wgpu::SwapChainFrame>,

    pub buffers: HashMap<BufferId, Arc<wgpu::Buffer>>,
}

impl RenderResources {
    pub fn new(device:Arc<wgpu::Device>) -> RenderResources {
        RenderResources {
            device,
            main_surface:None,
            main_swap_chain:None,
            main_swap_chain_frame:None,
            buffers:HashMap::default(),
        }
    }

    pub fn set_main_surface(&mut self,surface:wgpu::Surface) {
        self.main_surface = Some(surface);
    }

    pub fn create_buffer(&mut self,desc:&wgpu::BufferDescriptor) -> BufferId {
        let buffer = self.device.create_buffer(desc);
       
        let id = BufferId::new();
        self.buffers.insert(id, Arc::new(buffer));
        id
    }

    pub fn remove_buffer(&mut self,id:BufferId) {
        self.buffers.remove(&id);
    }

    pub fn map_buffer(&mut self,id:BufferId,mode:wgpu::MapMode) {
        let buffer = self.buffers.get(&id).unwrap();
        let buffer_slice = buffer.slice(..);
        let data = buffer_slice.map_async(mode);
        self.device.poll(wgpu::Maintain::Wait);
        if futures_lite::future::block_on(data).is_err() {
            panic!("Failed to map buffer to host.");
        }
    }

    pub fn unmap_buffer(&self, id: BufferId) {
        let buffer =self.buffers.get(&id).unwrap();
        buffer.unmap();
    }

    pub fn write_mapped_buffer(&self,id:BufferId,range:Range<u64>,write:&mut dyn FnMut(&mut [u8],&RenderResources)) {
        let buffer = self.buffers.get(&id).unwrap();
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

    pub fn next_swap_chain_texture(&mut self) {
        if let Some(swap_chain) = self.main_swap_chain.as_mut() {
            match swap_chain.get_current_frame() {
                Ok(frame) => {
                    self.main_swap_chain_frame = Some(frame)
                },
                Err(err) => panic!("{}",err)
            }
        }
    }

    pub fn clear_swap_chain_texture(&mut self) {
        self.main_swap_chain_frame = None;
    }
}