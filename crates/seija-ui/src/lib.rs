use event::UIEvent;
use seija_app::{IModule, App};
use seija_asset::AddAsset;
use seija_core::{CoreStage, AddCore}; 
use seija_app::ecs::prelude::*;
pub mod types;
pub mod text;
pub mod components;
pub mod mesh2d;
pub mod event;
mod render;
pub mod layout;
use components::ui_canvas::update_ui_canvas;
pub use render::update_ui_render;

use text::{FontLoader, Font};
#[derive(Clone, Copy,Hash,Debug,PartialEq, Eq,StageLabel)]
pub enum UIStageLabel {
    AfterStartup
}

pub struct UIModule;

impl IModule for UIModule {
    fn init(&mut self,app:&mut App) {
        //app.world.insert_resource(SpriteAllocator::new());
        app.add_event::<UIEvent>();
        app.add_asset::<Font>();
        app.add_asset_loader::<Font,FontLoader>();
        
         /*
        ui_layout_system
        ui_update_zorders
        update_ui_text
        ui_render_system
        ui_event_system
        update_transform_system
        */
         //app.add_system2(CoreStage::Startup,StartupStage::PostStartup, on_ui_start);
         app.add_system(CoreStage::PreUpdate,update_ui_canvas);
        //app.add_system(CoreStage::PostUpdate, update_ui_text.before(ui_render_system).after(ui_update_zorders));
        //app.add_system(CoreStage::PostUpdate, ui_layout_system.before(ui_update_zorders));
        //app.add_system(CoreStage::PostUpdate, ui_update_zorders.before(update_transform_system));
        //app.add_system(CoreStage::PostUpdate, ui_render_system.after(ui_update_zorders).before(update_transform_system));
        //app.add_system(CoreStage::PostUpdate, event::ui_event_system.after(ui_render_system));

    }
}


