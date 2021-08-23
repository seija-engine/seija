use seija_app::IModule;

pub mod core;
use seija_app::{App};

pub struct RenderModule(pub Box<dyn core::IBackend>);

impl IModule for RenderModule {
    fn init(&mut self,app:&mut App) {
    }
}