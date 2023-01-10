use bevy_ecs::schedule::RunOnce;
use seija_app::{IModule, App};
use seija_core::CoreStage; 
use seija_app::ecs::prelude::*;
use seija_transform::TransformLabel;
pub mod types;
mod sprite_alloc;
pub mod components;
pub mod mesh2d;
mod system;
mod root_render;
use components::ui_canvas::update_ui_canvas;
pub use sprite_alloc::system::update_sprite_alloc_render;
pub use sprite_alloc::alloc::SpriteAllocator;
use system::{UISystemData, on_after_start, update_render_system};
#[derive(Clone, Copy,Hash,Debug,PartialEq, Eq,StageLabel)]
pub enum UIStageLabel {
    AfterStartup
}

pub struct UIModule;

impl IModule for UIModule {
    fn init(&mut self,app:&mut App) {
        app.world.insert_resource(SpriteAllocator::new());
        app.init_resource::<UISystemData>();
        app.schedule.add_stage_after(CoreStage::Startup, UIStageLabel::AfterStartup, 
                                     SystemStage::single(on_after_start.exclusive_system())
                                     .with_run_criteria(RunOnce::default()));
        app.add_system(CoreStage::PostUpdate, update_render_system.exclusive_system().after(TransformLabel::Propagate));
        app.add_system(CoreStage::PreUpdate,update_ui_canvas.exclusive_system());
    }
}




