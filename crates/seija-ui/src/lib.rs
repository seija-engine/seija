use event::{UIEvent, ui_event_system};
use layout::system::ui_layout_system;
use render::WriteFontAtlas;
use seija_app::{IModule, App};
use seija_asset::{AddAsset, AssetStage};
use seija_core::{CoreStage, AddCore, StartupStage}; 
use seija_app::ecs::prelude::*;
pub mod types;
pub mod text;
pub mod components;
pub mod mesh2d;
pub mod event;
mod render;
mod system;
pub mod layout;
use components::ui_canvas::update_ui_canvas;
pub use render::update_ui_render;

use seija_render::RenderStage;
use seija_transform::update_transform_system;
use system::{on_ui_start, update_render_mesh_system, update_canvas_render, update_canvas_trans};
use text::{FontLoader, Font};
#[derive(Clone, Copy,Hash,Debug,PartialEq, Eq,StageLabel)]
pub enum UIStage {
    PreUI,
    UI,
    PostUI
}

pub struct UIModule;

impl IModule for UIModule {
    fn init(&mut self,app:&mut App) {
        //app.world.insert_resource(SpriteAllocator::new());
        app.add_event::<UIEvent>();
        app.add_event::<WriteFontAtlas>();
        app.add_asset::<Font>();
        app.add_asset_loader::<Font,FontLoader>();
        app.add_system2(CoreStage::Startup,StartupStage::PostStartup, on_ui_start);
        app.add_system(CoreStage::PreUpdate,update_ui_canvas);


        app.schedule.add_stage_before(AssetStage::AssetEvents, UIStage::PostUI, SystemStage::single_threaded());
        app.schedule.add_stage_before(UIStage::PostUI, UIStage::UI, SystemStage::single_threaded());
        app.schedule.add_stage_before(UIStage::UI, UIStage::PreUI, SystemStage::single_threaded());

        
        app.add_system(UIStage::PreUI,ui_layout_system.before(update_render_mesh_system));
        app.add_system(UIStage::PreUI, update_render_mesh_system);
        app.add_system(UIStage::UI, update_canvas_render);
        app.add_system(UIStage::UI, update_canvas_trans);
        app.add_system(UIStage::PostUI, ui_event_system);

        
         /*
        ui_layout_system
        ui_update_zorders
        update_ui_text
        ui_render_system
        ui_event_system
        update_transform_system
        */
        
        //app.add_system(CoreStage::PostUpdate, update_ui_text.before(ui_render_system).after(ui_update_zorders));
        //app.add_system(CoreStage::PostUpdate, ui_layout_system.before(ui_update_zorders));
        //app.add_system(CoreStage::PostUpdate, ui_update_zorders.before(update_transform_system));
        //app.add_system(CoreStage::PostUpdate, ui_render_system.after(ui_update_zorders).before(update_transform_system));
        //app.add_system(CoreStage::PostUpdate, event::ui_event_system.after(ui_render_system));
        
    }
}


