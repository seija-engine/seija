use std::{cell::RefMut, ops::Sub};

use lite_clojure_eval::EvalRT;
use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use glam::{Quat, Vec3};
use seija_app::App;
use seija_asset::{AssetEvent, AssetModule, Assets, Handle, HandleId};
use seija_core::{CoreModule, CoreStage, StartupStage, event::EventReader};
use seija_render::{RenderModule, camera::{camera::{Camera, Orthographic, Perspective}}, material::{Material, MaterialStorage, RenderOrder, read_material_def}, resource::{shape::Cube,Mesh}};
use seija_winit::WinitModule;
use seija_transform::{Transform, TransformModule, hierarchy::Parent};

fn main() {
    env_logger::init();
    let mut app = App::new();
    app.add_module(CoreModule);
    app.add_module(WinitModule::default());
    app.add_module(TransformModule);
    app.add_module(AssetModule);
    app.add_module(RenderModule);
    app.add_system2(CoreStage::Startup,StartupStage::Startup, on_start_up.system());
    app.add_system(CoreStage::Update, on_update.system());
    app.run();
}

pub struct  RootComponent {
    number:i32
}

fn on_start_up(mut commands:Commands,mut meshs:ResMut<Assets<Mesh>>,storage:Res<MaterialStorage>) {
    let root = {
        let mut root = commands.spawn();
        let t = Transform::default();
        root.insert(t);
        let mut per = Perspective::default();
        per.aspect_ratio = 640f32 / 480f32;
        let camera = Camera::from_3d(per);
        root.insert(camera);

        
        let test = RootComponent {number: 0};
        root.insert(test);
        root.id()
    };
    
    let test_md_string = std::fs::read_to_string("res/material/ui.md.clj").unwrap();
    let mut vm = EvalRT::new();
    let material_def = read_material_def(&mut vm, &test_md_string).unwrap();
  
    storage.add_def(material_def);
  
    create_elem(&mut commands, Vec3::new(8f32, 0f32, -20f32), root,&mut meshs,&storage);

    create_elem(&mut commands, Vec3::new(-8f32, 0f32, -20f32), root,&mut meshs,&storage);
}





fn create_elem(commands:&mut Commands,pos:Vec3,parent:Entity,meshs:&mut Assets<Mesh>,
               storage:&Res<MaterialStorage>) -> Entity {
    let mut elem = commands.spawn();
    let mut t = Transform::default();
    t.local.position = pos;
    t.local.rotation = Quat::from_rotation_y(45f32);
    elem.insert(t);
    elem.insert(Parent(parent));

    let cube = Cube::new(1.9f32);
    let cube_mesh:Mesh = cube.into();   
    let cube_mesh_handle = meshs.add( cube_mesh);
    
    let material = storage.create_material("ui-color").unwrap();
   
    elem.insert(cube_mesh_handle);
    elem.insert(material);
    elem.id()
}

fn on_update(mut commands:Commands,mut renders:Query<(Entity,&mut Transform,&Handle<Mesh>,&Handle<Material>)>,mats:Query<(Entity,&Handle<Material>)>) {
    for (e,mut t,_,_) in renders.iter_mut() {
        if t.local.position.x == 8f32 {
            let v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 3600) as f32;
       
            t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, v * 0.1f32 * 0.0174533f32, 0f32);
        }
        
       
       
    }
}