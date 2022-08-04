use bevy_ecs::system::{Commands, ResMut};
use glam::{Vec3, Quat, Vec4};
use seija_asset::Assets;
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::{init_core_app, add_pbr_camera, load_material};
use seija_gltf::{load_gltf, create_gltf};
use seija_pbr::lights::PBRLight;
use seija_render::{resource::{Mesh,Texture}, material::MaterialStorage};
use bevy_ecs::prelude::*;
use seija_transform::Transform;
pub fn main() {
    let mut app = init_core_app("model_render.clj");
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.run();
}

fn start(mut commands:Commands,
         window:Res<AppWindow>,
         mut meshs:    ResMut<Assets<Mesh>>,
         mut textures: ResMut<Assets<Texture>>,
         materials: Res<MaterialStorage>) {
    add_pbr_camera(&mut commands,&window,Vec3::new(0f32, -0.2f32, 2f32),Quat::IDENTITY,None);
    load_material("res/materials/baseTexture.mat.clj", &materials);
    let asset = load_gltf("res/gltf/shiba/scene.gltf",&mut meshs,&mut textures, None, None, None).unwrap();
    //light
    {
        let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 62000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::default()  , 90f32.to_radians(),  45f32.to_radians(), 0f32.to_radians());
        t.local.rotation = r;
        let mut l = commands.spawn();
        l.insert(light);
        l.insert(t);
    };
    let _ = create_gltf(Vec3::new(0f32, 0f32,0f32), &asset, &mut commands,  &|gltf_mat| {
        materials.create_material_with("baseTexture", |mat| {
            mat.texture_props.set("mainTexture", gltf_mat.base_color_texture.as_ref().unwrap().clone());
            
            mat.props.set_float4("color", Vec4::from_slice(&gltf_mat.base_color), 0);
            mat.props.set_float4("color", Vec4::new(0.7f32, 0.7f32, 0.7f32, 1f32), 0);
        })
    });
   
   
}