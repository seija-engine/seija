use bevy_ecs::prelude::World;
use seija_asset::{AssetServer, Assets};
use seija_core::{CoreStage, StartupStage};
use seija_examples::{init_core_app, load_material};
use seija_app::ecs::prelude::*;
use seija_skeleton3d::Skeleton3dModule;
use seija_template::Template;

struct GameData {
    root:Option<Entity>
}

pub fn main() {
    let mut app = init_core_app("render.clj");
    app.add_module(Skeleton3dModule);
    app.add_system2(CoreStage::Startup, StartupStage::Startup, on_start.exclusive_system());
    app.run();
}

fn on_start(world:&mut World) {
    let asset_server = world.get_resource::<AssetServer>().unwrap().clone();
    let req = asset_server.load_sync::<Template>(world,"template/posteffect.xml", None).unwrap();

    let templates = world.get_resource::<Assets<Template>>().unwrap();
    let template = templates.get(&req.id).unwrap();
    let e = Template::instance(template.clone(), world).unwrap();
    let game_data = GameData {root:Some(e) };
    world.insert_resource(game_data);
}