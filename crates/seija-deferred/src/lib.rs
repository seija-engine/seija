mod render_plugin;
mod deferred_light_pass;

use std::path::PathBuf;

use bevy_ecs::prelude::World;
pub use render_plugin::{create_deferred_plugin};
use seija_app::IModule;
use seija_core::LogOption;
use seija_render::material::MaterialStorage;

pub struct DeferredRenderModule {
    pub mat_path:String
}

impl IModule for DeferredRenderModule {
    fn init(&mut self,_app:&mut seija_app::App) {}

    fn start(&self, world:&mut World) {
       if self.load_res(world).is_none() {
           log::error!("read deferred material error");
       }
    }
}

impl DeferredRenderModule {
    pub fn load_res(&self,world:&mut World) -> Option<()> {
        let mats = world.get_resource::<MaterialStorage>()?;
        let mat_string = std::fs::read_to_string(self.mat_path.as_str())
                                        .ok().log_err(&format!("read file error:{}",self.mat_path.as_str()))?;
        mats.load_material_def(mat_string.as_str());
        Some(())
    }
}