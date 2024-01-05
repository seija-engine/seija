use bevy_ecs::schedule::{StageLabel, SystemStage};
use seija_app::{IModule, App};
use seija_asset::AssetStage;
pub mod common;
mod system;
mod components;

#[derive(Clone, Copy,Hash,Debug,PartialEq, Eq,StageLabel)]
pub enum R2DStage {
    R2D
}

pub struct R2DModule;

impl IModule for R2DModule {
    fn init(&mut self,app:&mut App) {
        app.schedule.add_stage_before(AssetStage::AssetEvents, R2DStage::R2D, SystemStage::single_threaded());
        app.add_system(R2DStage::R2D, system::image_system::image_system)
    }
}