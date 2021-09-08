use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query};
use glam::Vec3;
use seija_app::App;
use seija_core::{CoreModule, CoreStage, StartupStage};
use seija_render::{RenderModule, camera::{self, camera::{Camera, Orthographic, Projection}}, material::{Material, RenderOrder}};
use seija_winit::WinitModule;
use seija_transform::{Transform, TransformModule, hierarchy::Parent};

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

fn on_start_up(mut commands:Commands) {
    let root = {
        let mut root = commands.spawn();
        let t = Transform::default();
        root.insert(t);
        let camera = Camera::from_2d(Orthographic::default());
        root.insert(camera);
        root.id()
    };

    
    create_elem(&mut commands, Vec3::new(2f32, 2f32, 2f32), root);
    create_elem(&mut  commands, Vec3::new(1f32, 1f32, 1f32), root);

}


fn create_elem(commands:&mut Commands,pos:Vec3,parent:Entity) -> Entity {
    let mut elem = commands.spawn();
    let mut t = Transform::default();
    t.local.position = pos;
    elem.insert(t);
    let mat = Material {order : RenderOrder::Transparent};
    elem.insert(mat);
    elem.insert(Parent(parent));
    elem.id()
}

fn on_update(childrens:Query<(Entity,&Transform)>) {
   for (e,t) in childrens.iter() {
      //println!("e:{:?}  local:{:?} global:{:?}",e.id(),&t.local.position,&t.global().position);
   }
}