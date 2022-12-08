use bevy_ecs::{prelude::*, system::Command};
use glam::Vec4;
use seija_core::{CoreStage, StartupStage};
use seija_examples::{init_core_app};
use seija_transform::{Transform, PushChildren};
use seija_ui::components::{panel::Panel, rect2d::Rect2D, sprite::Sprite};
use smallvec::SmallVec;


fn main() {
    let mut app = init_core_app("FRPRender.clj");
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start.exclusive_system());
    app.add_system(CoreStage::Update, on_update);
   
    app.run();
}

fn start(world:&mut World) {
    let mut rect2d = Rect2D::default();
    rect2d.width = 1024f32;
    rect2d.height = 768f32;
    let panel_id = world.spawn().insert(Panel::default()).insert(Transform::default()).insert(rect2d).id();

    let mut rect2d = Rect2D::default();
    rect2d.width = 100f32;
    rect2d.height = 100f32;
    let sprite_id = world.spawn().insert(Sprite::simple("sprite".into(), Vec4::ONE)).insert(Transform::default()).id();
    PushChildren {
        parent:panel_id,
        children:SmallVec::from_slice(&[sprite_id])
    }.write(world);

}

fn on_update() {

}