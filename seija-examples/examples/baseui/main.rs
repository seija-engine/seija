use bevy_ecs::prelude::*;
use seija_core::{CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_ui::components::panel::Panel;


fn main() {
    let mut app = init_core_app("FRPRender.clj");
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start.exclusive_system());
    app.add_system(CoreStage::Update, on_update);
   
    app.run();
}

fn start(world:&mut World) {
    //world.spawn().insert(Panel::default()).insert(value);
}

fn on_update() {

}