pub use font_system::FontSystem;
use seija_app::{IModule, App};
use seija_core::CoreStage;
use system::update_text_size_system;
mod font_system;
mod font;
pub mod text;
mod swash;
mod system;

pub struct TextModule;

impl IModule for TextModule {
    fn init(&mut self,app:&mut App) {
        app.add_resource(FontSystem::default());
        app.add_system(CoreStage::LateUpdate, update_text_size_system);
    }
}