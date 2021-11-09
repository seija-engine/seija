use std::path::Path;

use glam::{Quat, Vec3};
use seija_examples::IExamples;
use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use seija_asset::{Assets, Handle};
use seija_core::{CoreStage, StartupStage, window::AppWindow};

use seija_gltf::load_gltf;
use seija_render::{camera::camera::Camera, material::MaterialStorage, resource::{CubeMapBuilder, Mesh, Texture, shape::{Cube, Sphere}}};
use seija_transform::Transform;

pub struct LightTest;


impl IExamples for LightTest {
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
   
   let texture = Texture::from_bytes(&std::fs::read("res/texture/b.jpg").unwrap()).unwrap() ;
   let texture_handle = textures.add(texture);
   let mesh = Sphere::new(2f32);//Sphere::new(2f32);
   let mesh_handle = meshs.add(mesh.into());
   let material_handle = materials.create_material_with("light", |mat|  {
       mat.texture_props.set("mainTexture", texture_handle.clone())
       
   });
   let mut t = Transform::default();
   t.local.position = Vec3::new(0f32, 0f32, -10f32);
   commands.spawn()
           .insert(mesh_handle)
           .insert(material_handle.unwrap())
           .insert(t);
}

fn on_update(mut query:Query<(Entity,&Handle<Mesh>,&mut Transform)>) {
    let v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 36000) as f32;
    let r = v * 0.01f32 * 0.0174533f32;
    for (_,_,mut t) in query.iter_mut() {
        t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ  , r, r, 0f32);
    }
}