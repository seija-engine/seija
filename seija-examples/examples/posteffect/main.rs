use bevy_ecs::prelude::World;
use seija_asset::{AssetServer, Assets};
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::{init_core_app};
use seija_app::ecs::prelude::*;
use seija_pbr::PBRCameraInfo;
use seija_render::{camera::camera::{Perspective, Camera}, material::Material};
use seija_skeleton3d::Skeleton3dModule;
use seija_template::Template;
use seija_transform::Transform;
#[derive(Resource)]
struct GameData {
    root:Option<Entity>
}

pub fn main() {
    let mut app = init_core_app("FRPRender.clj",vec![]);
    app.add_module(Skeleton3dModule);
    app.add_system2(CoreStage::Startup, StartupStage::Startup, on_start);
    app.run();
}

fn on_start(world:&mut World) {
    let asset_server = world.get_resource::<AssetServer>().unwrap().clone();
    let h_tonemaping = asset_server.load_sync::<Material>(world, "mats/tonemap.json", None).unwrap();
    //add camera
    {
       let window = world.get_resource::<AppWindow>().unwrap();
       let w = window.width() as f32;
       let h = window.height() as f32;
       let mut camera_entity = world.spawn_empty();
       let mut t = Transform::default();
       camera_entity.insert(t); 
       
       let mut per = Perspective::default();
       per.far = 50f32;
       per.aspect_ratio = w / h;
       let mut camera = Camera::from_3d(per);
       camera.cull_type = -1;
       camera_entity.insert(camera);

       let pbr_camera = PBRCameraInfo::default();
       camera_entity.insert(pbr_camera);

       //let mut post_stack = PostEffectStack::default();
       //post_stack.add(h_tonemaping);
       //camera_entity.insert(post_stack);
    };

    let req = asset_server.load_sync::<Template>(world,"template/posteffect.xml", None).unwrap();

    let templates = world.get_resource::<Assets<Template>>().unwrap();
    let template = templates.get(&req.id).unwrap();
    let e = Template::instance(template.clone(), world).unwrap();
    let game_data = GameData {root:Some(e) };
    world.insert_resource(game_data);
}