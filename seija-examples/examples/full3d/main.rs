use bevy_ecs::{prelude::*, system::{EntityCommands, CommandQueue}};
use glam::{Quat, Vec3};
use seija_asset::{AssetServer, Assets, Handle};
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::{init_core_app, update_camera_trans_system, add_pbr_camera};
use seija_pbr::lights::PBRLight;
use seija_render::{shadow::{ShadowLight, Shadow}, resource::{Mesh, shape::Sphere}, material::Material};
use seija_template::Template;
use seija_transform::Transform;

struct DemoGame {
    entity:Option<Entity>,
    template:Handle<Template>
}

fn main() {
    let mut app = init_core_app("FRPRender.clj");
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start.exclusive_system());
    app.add_system(CoreStage::Update, update_camera_trans_system);
    app.add_system(CoreStage::Update, on_update);
   
    app.run();
}

fn start(world:&mut World) {
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, world);
    let window = world.get_resource::<AppWindow>().unwrap();
    let camera_pos = Vec3::new(0f32, -0.2f32, 2f32);
    let r = Quat::IDENTITY;
    add_pbr_camera(&mut commands, window,camera_pos ,r , |_| {}, Some(70f32), None, false);
    queue.apply(world);
     //light
     {
        let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 92000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::default()  , -90f32.to_radians(),  35f32.to_radians(), 0f32.to_radians());
        t.local.rotation = r;
        let mut l = world.spawn();
        l.insert(light);
        l.insert(t);
        let mut shadow = ShadowLight::default();
        shadow.bias = 0.005f32;
        shadow.strength = 0.76f32;
        l.insert(shadow);
    };

   

    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut handle = server.load_sync::<Template>(world,"template/winter_scene/low_poly_winter_scene.xml", None).unwrap();
   

    let templates = world.get_resource::<Assets<Template>>().unwrap();
    let template = templates.get(&handle.id).unwrap();
    let entity = Template::instance(template.clone(), world).unwrap();
    let demo_game = DemoGame { entity:Some(entity),template:handle };
    world.insert_resource(demo_game);
}

fn on_update() {

}