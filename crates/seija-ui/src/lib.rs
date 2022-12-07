use seija_app::{IModule, App};
use seija_core::CoreStage; 
pub mod types;
mod sprite_alloc;
pub mod components;

pub struct UIModule;

impl IModule for UIModule {
    fn init(&mut self,app:&mut App) {
        app.add_system(CoreStage::PostUpdate, update_render_system);
    }
}

fn update_render_system() {

}