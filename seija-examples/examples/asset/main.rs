use bevy_ecs::system::{Commands, ResMut};
use glam::{Vec3, Quat};
use seija_asset::{Assets, AssetServer, Handle, AssetRequest};
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::{init_core_app, add_pbr_camera,  update_camera_trans_system};

use seija_pbr::lights::PBRLight;
use seija_render::{resource::{Mesh, shape::{Cube}, Texture}, material::{Material, MaterialDefineAsset}};
use bevy_ecs::prelude::*;
use seija_transform::Transform;

#[derive(Default)]
pub struct LocalData {
    _loading_track:Option<AssetRequest>
}


pub fn main() {
    let mut app = init_core_app("FRPRender.clj",vec![]);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, pre_start);
    app.add_system(CoreStage::Update, update_camera_trans_system);
    app.add_system(CoreStage::Update, on_update);
    app.add_system2(CoreStage::Startup, StartupStage::Startup, on_start.exclusive_system());
    app.init_resource::<LocalData>();
    app.run();
}

fn on_start(world:&mut World) {
    
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    server.load_sync::<MaterialDefineAsset>(world, "materials/pbrColor.mat.clj",None)
                     .map(|mut v| v.forget()).unwrap();
    server.load_sync::<MaterialDefineAsset>(world, "materials/baseTexture.mat.clj",None)
                    .map(|mut v| v.forget()).unwrap();
    let h_texture:Handle<Texture> = server.load_sync::<Texture>(world,"texture/b.jpg",None).unwrap();
    //Cube 
    /* 
    {
        let mut meshs = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh =  Cube::new(1f32);
        let hmesh = meshs.add(mesh.into());

        let mut material = Material::from_world(world, "materials/pbrColor.mat.clj").unwrap();
        material.props.set_float4("color", Vec4::ONE, 0);
        let mut materials = world.get_resource_mut::<Assets<Material>>().unwrap();
        let hmat = materials.add(material);

        let mut t = Transform::default();
        t.local.scale = Vec3::new(1f32, 1f32, 1f32);
        t.local.position = Vec3::new(1f32, 0f32, -0.5f32);
        t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, -31f32.to_radians(), 0f32);
       
        world.spawn().insert(hmesh).insert(hmat).insert(t);
    };*/

    //Cube
    {
        let mut meshs = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh =  Cube::new(1f32);
        let hmesh = meshs.add(mesh.into());

        let mut material = Material::from_world(world, "materials/baseTexture.mat.clj").unwrap();
        material.texture_props.set("mainTexture", h_texture.clone());
        let mut materials = world.get_resource_mut::<Assets<Material>>().unwrap();
        let hmat = materials.add(material);

        let mut t = Transform::default();
        t.local.scale = Vec3::new(2f32, 1f32, 1f32);
        t.local.position = Vec3::new(1f32, 0f32, -0.5f32);
        t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, -31f32.to_radians(), 0f32);
       
        world.spawn().insert(hmesh).insert(hmat).insert(t);
    };

}

fn pre_start(mut commands:Commands,mut _local_data:ResMut<LocalData>,_:Res<AssetServer>,window:Res<AppWindow>) {
    add_pbr_camera(&mut commands,&window,Vec3::new(0f32, 0f32, 2f32),Quat::IDENTITY,|_| {},None,None,false);
   
    //light
    {
        let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 62000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::default()  , 90f32.to_radians(),  45f32.to_radians(), 0f32.to_radians());
        t.local.rotation = r;
        let mut l = commands.spawn();
        l.insert(light);
        l.insert(t);
    }

   
   
}

fn on_update(mut _local_data:ResMut<LocalData>) {
    
}