use bevy_ecs::prelude::World;
use seija_asset::Assets;

use crate::{graph::INode, RenderContext, resource::{RenderResourceId, TextureDescInfo, Texture}};

use super::WindowEvent;

pub struct ScreenTextureNode {
    texture_descs:Vec<wgpu::TextureDescriptor<'static>>,
    win_event:WindowEvent,
    out_textures:Vec<Option<RenderResourceId>>
}

impl ScreenTextureNode {
    pub fn new(texture_descs:Vec<wgpu::TextureDescriptor<'static>>) -> ScreenTextureNode {
        let len = texture_descs.len();
        ScreenTextureNode { 
            texture_descs,
            win_event:WindowEvent::default(),
            out_textures:vec![None;len]
         }
    }
}

impl INode for ScreenTextureNode {
    fn output_count(&self) -> usize { self.texture_descs.len() }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,_:&Vec<Option<RenderResourceId>>,outputs:&mut Vec<Option<RenderResourceId>>) {
       if let Some((w,h)) = self.win_event.get_new_window_size(world) {
           
            for old_texture in self.out_textures.iter() {
                if let Some(id) = old_texture {
                    ctx.resources.remove_texture(id);
                }
            }

            let mut textures = world.get_resource_mut::<Assets<Texture>>().unwrap();
            let mut index:usize = 0;
            for texture_desc in self.texture_descs.iter_mut() {
                let mut desc = TextureDescInfo::default();
                desc.desc = texture_desc.clone();
                desc.desc.size.width = w;
                desc.desc.size.height = h;
                let rt_texture = Texture::create_by_desc(desc);
                let h_texture = textures.add(rt_texture);
                
                self.out_textures[index] = Some(RenderResourceId::Texture(h_texture));
                index += 1;
            }
       }
       *outputs = self.out_textures.clone();
    }
}