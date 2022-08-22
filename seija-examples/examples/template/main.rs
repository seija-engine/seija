use glam::{Vec3, Quat};
use seija_asset::{AssetServer};
use seija_core::{CoreStage, StartupStage};
use seija_examples::{init_core_app, update_camera_trans_system, load_material};
use bevy_ecs::prelude::*;
use seija_pbr::lights::PBRLight;
use seija_template::{Template, instance_template_sync};
use seija_transform::Transform;
pub fn main() {
    let mut app = init_core_app("model_render.clj");
    app.add_system2(CoreStage::Startup, StartupStage::Startup, start.exclusive_system());
    app.add_system(CoreStage::Update, update_camera_trans_system);
    app.run();
}

fn start(world:&mut World) {
    load_material("materials/pbrColor.mat.clj", world);

     //light
     {
        let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 62000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::default()  , -30f32.to_radians(),  50f32.to_radians(), 0f32.to_radians());
        t.local.rotation = r;
        let mut l = world.spawn();
        l.insert(light);
        l.insert(t);
    }

    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let tmpl_path = asset_server.full_path("template").unwrap().join("first.xml");
    let str_template = std::fs::read_to_string(&tmpl_path).unwrap();
    let tempalte = Template::from_str(&str_template).unwrap();
    if let Err(err) = instance_template_sync(world, &tempalte) {
        log::error!("err:{:?}",err);
    }
    
}