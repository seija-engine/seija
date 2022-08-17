use seija_asset::{AssetServer};
use seija_core::{CoreStage, StartupStage};
use seija_examples::{init_core_app, update_camera_trans_system};
use bevy_ecs::prelude::*;
use seija_template::{Template, instance_template_sync};
pub fn main() {
    let mut app = init_core_app("model_render.clj");
    app.add_system2(CoreStage::Startup, StartupStage::Startup, start.exclusive_system());
    app.add_system(CoreStage::Update, update_camera_trans_system);
    app.run();
}

fn start(world:&mut World) {
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let tmpl_path = asset_server.full_path("template").unwrap().join("first.xml");
    let str_template = std::fs::read_to_string(&tmpl_path).unwrap();
    let tempalte = Template::from_str(&str_template).unwrap();
    instance_template_sync(world, &tempalte);
    
}