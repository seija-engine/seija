use std::sync::Arc;

use bevy_ecs::{schedule::{StageLabel, SystemStage}, world::World, system::Resource};
use seija_app::{IModule, App};
use seija_asset::{AssetStage, AssetServer, Assets};
use seija_core::{CoreStage, StartupStage};
use seija_render::material::{MaterialDef, MaterialDefineAsset};
pub mod common;
mod system;
pub mod components;
pub mod ffi;

#[derive(Clone, Copy,Hash,Debug,PartialEq, Eq,StageLabel)]
pub enum R2DStage {
    R2D
}

pub struct R2DModule;

impl IModule for R2DModule {
    fn init(&mut self,app:&mut App) {
        app.schedule.add_stage_before(AssetStage::AssetEvents, R2DStage::R2D, SystemStage::single_threaded());
        app.add_system(R2DStage::R2D, system::render_system::image_and_sprite_system);
        app.add_system2(CoreStage::Startup,StartupStage::PostStartup, on_2d_start);
        app.add_system(CoreStage::PreUpdate, components::screen_scaler::screen_scaler_system);
    }
}

#[derive(Resource)]
pub struct Module2DResource {
   pub(crate) image_material_define:Arc<MaterialDef>,
   pub(crate) sprite_material_define:Arc<MaterialDef>,
}

fn on_2d_start(world:&mut World) {
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut h_image = server.load_sync::<MaterialDefineAsset>(world, "materials/ui.mat.clj", None).unwrap();
    h_image.forget();
    let mut h_sprite = server.load_sync::<MaterialDefineAsset>(world, "materials/sprite.mat.clj", None).unwrap();
    h_sprite.forget();

    let mats = world.get_resource::<Assets<MaterialDefineAsset>>().unwrap();
    let image_material_define = mats.get(&h_image.id).unwrap().define.clone();
    let sprite_material_define = mats.get(&h_sprite.id).unwrap().define.clone();
    let res2d = Module2DResource { image_material_define,sprite_material_define };
    world.insert_resource(res2d);
}