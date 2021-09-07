use std::ops::AddAssign;

use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query};
use glam::Vec3;
use seija_app::App;
use seija_core::{CoreModule, CoreStage, StartupStage};
use seija_render::{RenderModule, camera::{self, camera::{Orthographic, Projection}}};
use seija_winit::WinitModule;
use seija_transform::{Transform, TransformModule, hierarchy::{Children,Parent}};

fn main() {
    let mut app = App::new();
    app.add_module(CoreModule);
    app.add_module(WinitModule::default());
    app.add_module(TransformModule);
    app.add_module(RenderModule);
    app.add_system2(CoreStage::Startup,StartupStage::Startup, on_start_up.system());
    app.add_system(CoreStage::Update, on_update.system());
    app.run();
}

fn on_start_up(mut cmds:Commands) {
   let root = {
    let mut t = Transform::default();
    t.local.position = Vec3::ONE;
    cmds.spawn().insert(t).id()
   };
   let mut a = cmds.spawn();
   let mut ct = Transform::default();
   ct.local.position = Vec3::new(2f32, 2f32, 2f32);
   a.insert(ct);
   let proj = Projection::Ortho(Orthographic::default());
   a.insert(camera::camera::Camera {projection :proj});
}


fn on_update(childrens:Query<(Entity,&Transform)>) {
   for (e,t) in childrens.iter() {
       println!("e:{:?}  local:{:?} global:{:?}",e.id(),&t.local.position,&t.global().position);
   }
}