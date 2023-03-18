use event::UIEvent;
use layout::system::ui_layout_system;
use seija_app::{IModule, App};
use seija_core::{CoreStage, StartupStage, AddCore}; 
use seija_app::ecs::prelude::*;
pub mod types;
mod sprite_alloc;
pub mod components;
pub mod mesh2d;
mod info;
mod system;
pub mod event;
pub mod layout;
use components::ui_canvas::update_ui_canvas;
use seija_transform::update_transform_system;
pub use sprite_alloc::system::update_sprite_alloc_render;
pub use sprite_alloc::alloc::SpriteAllocator;

use system::{ui_render_system, on_ui_start, ui_update_zorders};
#[derive(Clone, Copy,Hash,Debug,PartialEq, Eq,StageLabel)]
pub enum UIStageLabel {
    AfterStartup
}

pub struct UIModule;

impl IModule for UIModule {
    fn init(&mut self,app:&mut App) {
        app.world.insert_resource(SpriteAllocator::new());
        app.add_event::<UIEvent>();
        app.add_system2(CoreStage::Startup,StartupStage::PostStartup, on_ui_start);
        app.add_system(CoreStage::PreUpdate,update_ui_canvas);
        
        app.add_system(CoreStage::PostUpdate, ui_layout_system.before(ui_update_zorders));
        app.add_system(CoreStage::PostUpdate, ui_update_zorders.before(update_transform_system));
        app.add_system(CoreStage::PostUpdate, ui_render_system.after(ui_update_zorders).before(update_transform_system));
        app.add_system(CoreStage::PostUpdate, event::ui_event_system.after(ui_render_system));
    }
}



