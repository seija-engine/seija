use std::{sync::Arc};

#[derive(Debug,Clone,Hash,PartialEq, Eq)]
pub struct ResourceId;

pub struct RenderResources {
    pub device: Arc<wgpu::Device>,
    main_surface:Option<wgpu::Surface>,
    main_swap_chain:Option<wgpu::SwapChain>,
    main_swap_chain_frame:Option<wgpu::SwapChainFrame>
}

impl RenderResources {
    pub fn new(device:Arc<wgpu::Device>) -> RenderResources {
        RenderResources {
            device,
            main_surface:None,
            main_swap_chain:None,
            main_swap_chain_frame:None
        }
    }

    pub fn set_main_surface(&mut self,surface:wgpu::Surface) {
        self.main_surface = Some(surface);
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