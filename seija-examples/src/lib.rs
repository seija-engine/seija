use bevy_ecs::{prelude::{Commands, Entity, Res}};
use glam::{Quat, Vec3, Vec4};
use lite_clojure_eval::EvalRT;
use seija_app::App;
use seija_asset::{Assets, Handle};
use seija_core::{CoreStage, window::AppWindow};
use seija_pbr::PBRCameraInfo;
use seija_render::{camera::{camera::Perspective,camera::Camera}, material::{MaterialStorage, read_material_def}, resource::{Mesh, Texture, TextureDescInfo}};
use seija_transform::{Transform, hierarchy::Parent};
use bevy_ecs::prelude::*;
use seija_render::wgpu;

pub trait IExamples {
    fn run(app:&mut App);
}


pub fn pre_start(mut commands:Commands,window:Res<AppWindow>,mats:Res<MaterialStorage>) {
   
    //add_camera_3d(&mut commands, &window);
    //load_material("res/new_material/color.mat.clj", &mats);
    //load_material("res/new_material/texture.mat.clj", &mats);
    //load_material("res/new_material/bpColor.mat.clj", &mats);
    //load_material("res/new_material/pbrColor.mat.clj", &mats);
    //load_material("res/material/color/model_color.clj", &mats);
    //load_material("res/material/skybox/sky.clj", &mats);
    //load_material("res/material/light/light.clj", &mats);
    //load_material("res/material/pbr/pbr.clj", &mats);
}

pub fn add_camera_3d(mut commands:&mut Commands,window:&AppWindow) -> Entity {
    let mut root = commands.spawn();
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32, 20f32, 70f32);
    t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ , -15f32 *  0.0174533f32, 0f32, 0f32); 
    root.insert(t);
    
    let mut per = Perspective::default();
    per.aspect_ratio = window.width() as f32 / window.height() as f32;
    let camera = Camera::from_3d(per);
    root.insert(camera);

    root.id()
    
}

pub fn add_pbr_camera(window:&AppWindow,commands: &mut Commands) {
    
    let mut root = commands.spawn();
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32, 10f32, 0f32);
    t.local.rotation = Quat::IDENTITY; 
    root.insert(t);
    let mut per = Perspective::default();
    per.aspect_ratio = window.width() as f32 / window.height() as f32;
    let camera = Camera::from_3d(per);
    root.insert(camera);

    let pbr_camera = PBRCameraInfo::default();
    root.insert(pbr_camera);
}

pub fn load_material(path:&str,mats:&MaterialStorage) {
    println!("load_material:{}",path);
    let code_string = std::fs::read_to_string(path).unwrap();
    let mut vm = EvalRT::new();
    let mat_def = read_material_def(&mut vm, &code_string).unwrap();
    mats.add_def(mat_def);
}


pub fn load_texture(textures:&mut Assets<Texture>,path:&str) -> Handle<Texture> {
    let texture = Texture::from_image_bytes(&std::fs::read(path).unwrap(),TextureDescInfo::default()).unwrap();
    textures.add(texture)
}

pub fn add_render_mesh(
                       commands:&mut Commands,
                       mesh:Handle<Mesh>,
                       texture:Handle<Texture>,
                       mat_name:&str,
                       pos:Vec3,
                       mats:&MaterialStorage) -> Entity {
   
    let mut elem = commands.spawn();
    let mut t = Transform::default();
    t.local.position = pos;
    t.local.rotation = Quat::from_rotation_y(45f32);
    elem.insert(t);
    
    elem.insert(mesh);

    let material = mats.create_material(mat_name).unwrap();
   
    
    let mut mats = mats.mateials.write();
    let mat = mats.get_mut(&material.id).unwrap();
    elem.insert(material);
    mat.texture_props.set("mainTexture", texture);
    mat.props.set_float4("color", Vec4::new(1f32, 1f32, 1f32, 1f32), 0);
    
    
    elem.id()
}