use glam::{Vec3, Vec4};
use seija_examples::IExamples;
use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use seija_asset::{Assets};
use seija_core::{CoreStage, StartupStage, window::AppWindow};

use seija_render::{camera::camera::Camera, material::MaterialStorage, resource::{Mesh, Texture, shape::{Sphere, Cube}}};
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
      let mesh = Cube::new(2f32);
      let hmesh = meshs.add(mesh.into());
      let hmat = materials.create_material_with("purecolor",|mat| {
          mat.props.set_float4("color", Vec4::new(0f32, 1f32, 0f32, 1f32), 0);
      }).unwrap();
      let mut t = Transform::default();
      t.local.scale = Vec3::new(1f32, 1f32, 1f32);
      t.local.position = Vec3::new(2f32, 0f32, -10f32);
      commands.spawn()
              .insert(hmesh)
              .insert(hmat)
              .insert(t);
}

fn on_update(mut query:Query<(Entity,&Camera,&mut Transform)>) {

}