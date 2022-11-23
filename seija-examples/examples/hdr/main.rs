use bevy_ecs::system::{EntityCommands, CommandQueue};
use glam::{Vec3, Quat};
use seija_asset::{AssetServer, Assets};
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::{init_core_app, update_camera_trans_system, add_pbr_camera};
use seija_app::ecs::prelude::*;
use seija_pbr::lights::PBRLight;
use seija_template::Template;
use seija_transform::Transform;
fn main() {
    let mut app = init_core_app("FRPRender.clj");
    app.add_system2(CoreStage::Startup, StartupStage::Startup, on_start.exclusive_system());
    app.add_system(CoreStage::Update, update_camera_trans_system);

    app.run();
}

fn on_start(world:&mut World) {
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
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, world);
    let window = world.get_resource::<AppWindow>().unwrap();
    
    add_pbr_camera(&mut commands,&window,Vec3::new(0f32, 3.09f32, 6.18f32),Quat::IDENTITY,|_:&mut EntityCommands| {
       
    },None,None,false);
    queue.apply(world);
    

    let asset_server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut handle = asset_server.load_sync::<Template>(world,"template/snow_house.xml", None).unwrap();
    let templates = world.get_resource::<Assets<Template>>().unwrap();
    let template = templates.get(&handle.id).unwrap().clone();
    template.instance(world).unwrap();
    handle.forget();
}