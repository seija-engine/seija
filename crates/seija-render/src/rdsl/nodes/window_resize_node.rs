use bevy_ecs::prelude::World;
use lite_clojure_eval::Variable;
use seija_asset::Assets;
use anyhow::{Result};

use crate::{IUpdateNode, rdsl::{atom::Atom, win_event::WindowEvent}, resource::{RenderResourceId, Texture}, RenderContext};
#[derive(Default)]
pub struct WindowReSizeNode {
    textures:Vec<*mut Atom<RenderResourceId>>,
    win_event:WindowEvent
}

impl IUpdateNode for WindowReSizeNode {
    fn update_params(&mut self,params:Vec<Variable>) -> Result<()> {
       if let Some(list) = params[0].cast_vec() {
           for item in list.borrow().iter() {
                if let Some(u8_ptr) = item.cast_userdata() {
                    let ptr = u8_ptr as *mut Atom<RenderResourceId>;
                    self.textures.push(ptr);
                }
           }
       }
       Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
        if let Some((w,h)) = self.win_event.get_new_window_size(world) {
            for atom_texture in self.textures.iter() {
                let atom_mut = unsafe { &mut**atom_texture };
                ctx.resources.remove_texture(atom_mut.inner());
            }

            let mut textures = world.get_resource_mut::<Assets<Texture>>().unwrap();
            for atom_texture in self.textures.iter() {
                let atom_mut = unsafe { &mut**atom_texture };
                if let RenderResourceId::Texture(h_texture) = atom_mut.inner() {
                    if let Some(texture) = textures.remove(h_texture.id) {
                        let mut texture_desc = texture.desc().clone();
                        texture_desc.desc.size.width = w;
                        texture_desc.desc.size.height = h;

                        let new_h_texture = textures.add(Texture::create_by_desc(texture_desc));
                        atom_mut.set(RenderResourceId::Texture(new_h_texture));
                    }
                }
            }
        }
    }
}

impl WindowReSizeNode {
    pub fn _update(&self) {
        
    }
}