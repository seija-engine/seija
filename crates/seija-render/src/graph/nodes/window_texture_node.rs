use bevy_ecs::prelude::World;
use seija_core::{event::{Events, ManualEventReader}, window::{AppWindow}};
use seija_winit::event::{WindowCreated,WindowResized};
use crate::{RenderContext, graph::node::INode, resource::{RenderResourceId}};

pub struct WindowTextureNode {
    texture_res_id:Option<RenderResourceId>,
    desc:wgpu::TextureDescriptor<'static>,
    pub window_resized_event_reader: ManualEventReader<WindowResized>,
    pub window_created_event_reader: ManualEventReader<WindowCreated>,
}

impl WindowTextureNode {
    pub fn new(desc:wgpu::TextureDescriptor<'static>) -> WindowTextureNode {
        
        WindowTextureNode {
            texture_res_id:None,
            desc,
            window_resized_event_reader:Default::default(),
            window_created_event_reader:Default::default()
        }
    }
}

impl INode for WindowTextureNode {
    fn update(&mut self,world: &mut World,
                        ctx:&mut RenderContext,
                        _inputs:&Vec<Option<RenderResourceId>>,
                        outputs:&mut Vec<Option<RenderResourceId>>) {
        let mut is_make_texture:bool = false;

        if let Some(events) =  world.get_resource::<Events<WindowCreated>>() {
            
            is_make_texture =  self.window_created_event_reader.iter(events).count() > 0;
            
        }
        if !is_make_texture  {
            if let Some(events) =  world.get_resource::<Events<WindowResized>>() {
                is_make_texture = self.window_resized_event_reader.iter(events).count() > 0;
            }
        }
        
        
        if is_make_texture {
           if let Some(tid) = self.texture_res_id.take() {
                ctx.resources.remove_texture(&tid);
           }
           let app_window = world.get_resource::<AppWindow>().unwrap();
           if app_window.width() > 0 && app_window.height() > 0 {
            self.desc.size.width = app_window.width();
            self.desc.size.height = app_window.height();
            if ctx.setting.msaa_samples > 1 {
                self.desc.sample_count = ctx.setting.msaa_samples;
            } else {
                self.desc.sample_count = 1;
            }
            let texture_id = ctx.resources.create_texture(&self.desc, &wgpu::TextureViewDescriptor::default());
            self.texture_res_id = Some(RenderResourceId::Texture(texture_id));
           }
        }

        outputs[0] = self.texture_res_id.clone()
    }

    fn output_count(&self) -> usize { 1 }
}