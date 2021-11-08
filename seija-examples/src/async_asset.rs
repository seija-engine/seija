use std::path::Path;

use seija_examples::IExamples;
use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use seija_asset::{Assets};
use seija_core::{CoreStage, StartupStage, window::AppWindow};

use seija_gltf::load_gltf;
use seija_render::{camera::camera::Camera, material::MaterialStorage, resource::{CubeMapBuilder, Mesh, Texture}};
use seija_transform::Transform;

pub struct AsyncAsset;


impl IExamples for AsyncAsset {
    fn run(app:&mut seija_app::App) {
       app.add_system2(CoreStage::Startup,StartupStage::Startup, on_start.system());
       app.add_system(CoreStage::Update, on_update.system());
    }
}

fn on_start(mut commands:Commands,
    mut meshs:ResMut<Assets <Mesh>>,
    mut textures:ResMut<Assets<Texture>>,
    window:Res<AppWindow>,
    materials:Res<MaterialStorage>) {
}

fn on_update(mut query:Query<(Entity,&Camera,&mut Transform)>) {

}