use std::path::Path;

use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use glam::{Quat, Vec3, Vec4};
use seija_asset::{Asset, Assets, Handle, HandleId};
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::IExamples;
use seija_gltf::{create_gltf, load_gltf};
use seija_render::{material::MaterialStorage, resource::{CubeMapBuilder, Mesh, Texture, shape::Cube}};
use seija_transform::Transform;

pub struct CubeMapTest;

impl IExamples for CubeMapTest {
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

    let p = Path::new("res/texture/cubemap/Yokohama3/");
    let cube_builder = CubeMapBuilder::new();
    let texture = cube_builder.with_left_path(p.join("posx.jpg")).unwrap()
                .with_right_path(p.join("negx.jpg")).unwrap()
                .with_top_path(p.join("posy.jpg")).unwrap()
                .with_bottom_path(p.join("negy.jpg")).unwrap()
                .with_back_path(p.join("posz.jpg")).unwrap()
                .with_front_path(p.join("negz.jpg")).unwrap().build();
    let texture_handle = HandleId::random::<Texture>();
    textures.set_untracked(texture_handle,texture.unwrap());

    let mesh = Cube::new(2f32).into();
    let cube_mesh = meshs.add(mesh);
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32,0f32,-1f32);
    t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ , 0f32, 0.65f32, 0f32);
    
    let mat_handle = materials.create_material_with("sky-box", |mat| {
        mat.texture_props.set("mainTexture", Handle::weak(texture_handle));
    }).unwrap();
    commands.spawn().insert(t).insert(cube_mesh).insert(mat_handle);

    let fox_asset = load_gltf("res/gltf/Fox/glTF-Binary/Fox.glb",&mut meshs,
                                 &mut textures).unwrap();
   
     create_gltf(
  Vec3::new(0f32, -50f32, -300f32),
      &fox_asset, &mut commands,&|gltf_material| {
      if let Some(texture) = gltf_material.base_color_texture.as_ref() {
         materials.create_material_with("model",|mat| {
            mat.texture_props.set("mainTexture", texture.clone());
         })
      } else {
         materials.create_material_with("model-color",|mat| {
            mat.props.set_float4("color", Vec4::from(gltf_material.base_color), 0);
         }) 
      }
   });
}

fn on_update(mut query:Query<(Entity,&Handle<Mesh>,&mut Transform)>) {
    let v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 36000) as f32;
    for (e,m,mut t) in query.iter_mut() {
        let r = v * 0.01f32 * 0.0174533f32;
        t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ  , 0f32, r , 0f32);
    }
 }