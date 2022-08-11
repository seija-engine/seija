use bevy_ecs::system::{Commands, ResMut};
use glam::{Vec3, Quat, Vec4};
use seija_asset::{Assets, AssetServer, LoadingTrack, Handle};
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::{init_core_app, add_pbr_camera, load_material, update_camera_trans_system};
use seija_gltf::{create_gltf, asset::GltfAsset};
use seija_pbr::lights::PBRLight;
use seija_render::{ material::MaterialStorage};
use bevy_ecs::prelude::*;
use seija_transform::Transform;

#[derive(Default)]
struct GameData {
    shiba_asset:Option<Handle<GltfAsset>>,
    track:Option<LoadingTrack>
}

pub fn main() {
    let mut app = init_core_app("model_render.clj");
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.add_system(CoreStage::Update, update_camera_trans_system);
    app.add_system(CoreStage::Update, on_update);
    app.init_resource::<GameData>();
    app.run();
}

fn start(mut commands:Commands,
         mut data:ResMut<GameData>,
         window:Res<AppWindow>,
         server:Res<AssetServer>,
         materials: Res<MaterialStorage>) {
    add_pbr_camera(&mut commands,&window,Vec3::new(0f32, -0.2f32, 2f32),Quat::IDENTITY,None);
    load_material("res/materials/baseTexture.mat.clj", &materials);
    let gltf = server.load_async::<GltfAsset>("res/gltf/shiba/scene.gltf", None).unwrap();
    data.track = Some(gltf);
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
    /* 
    let _ = create_gltf(Vec3::new(0f32, 0f32,0f32), &asset, &mut commands,  &|gltf_mat| {
        materials.create_material_with("baseTexture", |mat| {
            mat.texture_props.set("mainTexture", gltf_mat.base_color_texture.as_ref().unwrap().clone());
            
            mat.props.set_float4("color", Vec4::from_slice(&gltf_mat.base_color), 0);
            mat.props.set_float4("color", Vec4::new(0.7f32, 0.7f32, 0.7f32, 1f32), 0);
        })
    });*/
   
   
}

fn on_update(mut commands:Commands,mut data:ResMut<GameData>,gltfs:Res<Assets<GltfAsset>>,materials: Res<MaterialStorage>) {
    let is_finish = data.track.as_ref().map(|v| v.is_finish()).unwrap_or(false);
    if is_finish {
       data.shiba_asset = data.track.as_ref().map(|v| v.clone_typed_handle());
       data.track = None;
       if let Some(asset) = gltfs.get(&data.shiba_asset.as_ref().unwrap().id) {
        let _ = create_gltf(Vec3::new(0f32, 0f32,0f32), &asset, &mut commands,  &|gltf_mat| {
            materials.create_material_with("baseTexture", |mat| {
                mat.texture_props.set("mainTexture", gltf_mat.base_color_texture.as_ref().unwrap().clone());
                
                mat.props.set_float4("color", Vec4::from_slice(&gltf_mat.base_color), 0);
                mat.props.set_float4("color", Vec4::new(0.7f32, 0.7f32, 0.7f32, 1f32), 0);
            })
        });
       }
    }
}