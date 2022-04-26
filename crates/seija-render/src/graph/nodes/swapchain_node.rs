use bevy_ecs::prelude::World;
use seija_core::{window::AppWindow, event::{ManualEventReader, Events}};
use seija_winit::event::{WindowResized, WindowCreated};

use crate::{RenderContext, graph::node::INode, resource::{RenderResourceId, RenderResources}};

pub struct SwapchainNode {
    mssa_texture:Option<RenderResourceId>,
    pub window_resized_event_reader: ManualEventReader<WindowResized>,
    pub window_created_event_reader: ManualEventReader<WindowCreated>,
}

impl SwapchainNode {
    pub fn new() -> SwapchainNode {
        SwapchainNode { mssa_texture:None, 
            window_resized_event_reader:Default::default(),
            window_created_event_reader:Default::default() 
        }
    }
}

impl INode for SwapchainNode {
    fn prepare(&mut self, world: &mut World, ctx:&mut RenderContext) {
        if ctx.setting.msaa_samples > 1 && self.mssa_texture.is_none() {
            let app_window = world.get_resource::<AppWindow>().unwrap();
            let w = app_window.width();
            let h = app_window.height();
            self.mssa_texture = Some(Self::create_msaa_texture(w, h, &mut ctx.resources, ctx.setting.msaa_samples));
        }

        if self.mssa_texture.is_some() {
            if let Some(events) =  world.get_resource::<Events<WindowResized>>() {
               let is_remake_texture = self.window_resized_event_reader.iter(events).count() > 0;
               if is_remake_texture {
                    let app_window = world.get_resource::<AppWindow>().unwrap();
                    let w = app_window.width();
                    let h = app_window.height();
                    if w > 0 && h > 0 {
                        self.mssa_texture = Some(Self::create_msaa_texture(w, h, &mut ctx.resources, ctx.setting.msaa_samples));
                    }
                    
               }
            }
        }
    }

    fn update(&mut self,world: &mut World,
                        ctx:&mut RenderContext,
                        inputs:&Vec<Option<RenderResourceId>>,
                        outputs:&mut Vec<Option<RenderResourceId>>) {
       if let Ok(res_id) = ctx.resources.next_swap_chain_texture() {
           outputs[0] = Some(res_id);
           outputs[1] = self.mssa_texture.clone();
       }
    }

    fn output_count(&self) -> usize { 2 }
}

impl SwapchainNode {
    pub fn create_msaa_texture(w:u32,h:u32,res:&mut RenderResources,msaa_count:u32) -> RenderResourceId {
        let tex_id = res.create_texture(&wgpu::TextureDescriptor {
            label:None,
            size:wgpu::Extent3d {width:w,height:h,depth_or_array_layers:1},
            mip_level_count:1,
            sample_count:msaa_count,
            dimension:wgpu::TextureDimension::D2,
            format:wgpu::TextureFormat::Bgra8UnormSrgb,
            usage:wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED
        }, &wgpu::TextureViewDescriptor::default());
        RenderResourceId::Texture(tex_id)
    }
}