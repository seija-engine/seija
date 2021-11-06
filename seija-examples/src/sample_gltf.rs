use glam::{Vec3, Vec4};
use seija_asset::{Assets, Handle};
use seija_core::{CoreStage, StartupStage, bytes::FromBytes, window::AppWindow};
use seija_examples::{IExamples, add_render_mesh, load_material};
use bevy_ecs::{prelude::{Commands, Entity, Query, Res, ResMut}, system::{IntoSystem,SystemParam}};
use seija_gltf::{create_gltf, load_gltf};
use seija_render::{camera::camera::{Camera, Perspective}, material::{Material, MaterialStorage}, resource::{Mesh, Texture}};
use seija_transform::{Transform, hierarchy::Parent};
use crate::lib::{add_camera_3d};
pub struct SampleGltf;

impl IExamples for SampleGltf {
    fn run(app:&mut seija_app::App) {
       app.add_system2(CoreStage::Startup,StartupStage::Startup, on_start.system());
       app.add_system(CoreStage::Update, on_update.system());
    }
}

#[derive(SystemParam)]
pub struct StartParam<'a> {
   pub commands: Commands<'a>,
   pub window:Res<'a,AppWindow>,
   pub meshs:ResMut<'a,Assets<Mesh>>,
   pub textures:ResMut<'a,Assets<Texture>>,
   pub materials:ResMut<'a,MaterialStorage>
}

#[derive(SystemParam)]
pub struct UpdateParam<'a> {
   pub view_query: Query<'a,(Entity,&'static Camera)> ,
}

fn on_start(mut commands:Commands,
            mut meshs:ResMut<Assets<Mesh>>,
            mut textures:ResMut<Assets<Texture>>,
            window:Res<AppWindow>,
            materials:Res<MaterialStorage>) {
    
    let gltf_asset = load_gltf("res/gltf/Fox/glTF-Binary/Fox.glb",
                                   &mut meshs,
                                 &mut textures).unwrap();
    let first_primtives = &gltf_asset.first_mesh().primitives[0];
    let gltf_mesh = first_primtives.mesh.clone();
    let gltf_texture = first_primtives.material.as_ref().unwrap().base_color_texture.as_ref().unwrap().clone();

    
    let mode_id = add_render_mesh(&mut commands, 
                    gltf_mesh,
                    gltf_texture,
           "model",
                Vec3::new(0f32, 0f32, -250f32),
               &materials);
   
   let buggy_asset = load_gltf("res/gltf/Buggy.glb",&mut meshs,&mut textures).unwrap();

   create_gltf(
  Vec3::new(0f32, -50f32, -200f32),
      &buggy_asset, &mut commands,&|gltf_material| {
      if let Some(texture) = gltf_material.base_color_texture.as_ref() {
         materials.create_material_with("model",|mat| {
            mat.texture_props.set("mainTexture", texture.clone());
         })
      } else {
         materials.create_material_with("model-color",|mat| {
            mat.props.set_float4("color", Vec4::from(gltf_material.base_color), 0);
         }) 
      }
   });/**/
   
}

fn on_update(params:UpdateParam) {
   //let v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 3600) as f32;
}