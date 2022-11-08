use bevy_ecs::world::World;
use lite_clojure_eval::Variable;
use lite_clojure_frp::{DynamicID, FRPSystem};
use anyhow::Result;
use seija_asset::Assets;
use crate::{dsl_frp::win_event::WindowEvent, RenderContext, resource::{RenderResourceId, Texture}};
use super::IUpdateNode;

pub struct WindowReSizeNode {
    dyn_textures:Vec<DynamicID>,
    win_event:WindowEvent
}

impl WindowReSizeNode {
    pub fn from_args(args:Vec<Variable>) -> Result<Box<dyn IUpdateNode>> {
        let mut dyn_textures = vec![];
        for arg in args.iter() {
            if let Variable::Int(id) = arg {
                let dyn_id = *id as DynamicID;
                dyn_textures.push(dyn_id);
            }
        }
        Ok(Box::new(WindowReSizeNode {
            dyn_textures,
            win_event:WindowEvent::default()
        }))
    }
}

impl IUpdateNode for WindowReSizeNode {
    fn update(&mut self,world:&mut World,_ctx:&mut RenderContext,frp_sys:&mut FRPSystem) ->Result<()> {
        
        if let Some((w,h)) = self.win_event.get_new_window_size(world) {
            let mut textures = world.get_resource_mut::<Assets<Texture>>().unwrap();
            for dyn_id in self.dyn_textures.iter() {
               if let Some(dynamic) = frp_sys.dynamics.get_mut(&dyn_id) {
                   if let Some(dynamic_ptr) = dynamic.get_value().cast_userdata() {
                    if let RenderResourceId::Texture(h_tex) = *unsafe { Box::from_raw(dynamic_ptr as *mut RenderResourceId) } {
                        if let Some(texture) = textures.remove(h_tex.id) {
                           let mut texture_desc = texture.desc().clone();
                           texture_desc.desc.size.width = w;
                           texture_desc.desc.size.height = h;
                           let new_h_texture = Box::new(RenderResourceId::Texture(textures.add(Texture::create_by_desc(texture_desc))));
                           let ptr = Box::into_raw(new_h_texture) as *mut u8;
                           dynamic.set_value(Variable::UserData(ptr));
                        }
                      }
                   }
               }
            }
        }
        Ok(())   
    }
}