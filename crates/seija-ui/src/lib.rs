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
pub mod ffi;
use components::ui_canvas::update_ui_canvas;
pub use render::update_ui_render;
use system::{on_ui_start, update_render_mesh_system, update_canvas_render, update_canvas_trans, update_ui_clips};
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

        /*
        CoreStage::First"
    "CoreStage::Startup"
    "AssetStage::LoadAssets"
    "CoreStage::PreUpdate"
    "CoreStage::Update"
    "CoreStage::PostUpdate"
    "UIStage::PreUI"
    "UIStage::UI"
    "UIStage::PostUI"
    "AssetStage::AssetEvents"
    "RenderStage::PreRender"
    "RenderStage::Render"
    "RenderStage::PostRender"
    "CoreStage::Last"
        */
        app.schedule.add_stage_before(AssetStage::AssetEvents, UIStage::PostUI, SystemStage::single_threaded());
        app.schedule.add_stage_before(UIStage::PostUI, UIStage::UI, SystemStage::single_threaded());
        app.schedule.add_stage_before(UIStage::UI, UIStage::PreUI, SystemStage::single_threaded());

        
        app.add_system(CoreStage::LateUpdate,ui_layout_system);
        app.add_system(UIStage::PreUI, update_render_mesh_system);
        app.add_system(UIStage::UI, update_canvas_render);
        app.add_system(UIStage::UI, update_canvas_trans.after(update_canvas_render));
        app.add_system(UIStage::PostUI, update_ui_clips);
        app.add_system(UIStage::PostUI, ui_event_system);
        
    }
}

