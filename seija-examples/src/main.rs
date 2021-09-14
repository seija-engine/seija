use lite_clojure_eval::EvalRT;
use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use glam::Vec3;
use seija_app::App;
use seija_asset::{AssetEvent, AssetModule, Assets, HandleId};
use seija_core::{CoreModule, CoreStage, StartupStage, event::EventReader};
use seija_render::{RenderModule,resource::{shape::Cube,Mesh} ,camera::{camera::{Camera, Orthographic}}, material::{Material, MaterialDef, RenderOrder, read_material_def}};
use seija_winit::WinitModule;
use seija_transform::{Transform, TransformModule, hierarchy::Parent};

fn main() {
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

fn on_start_up(mut commands:Commands,mut mat_defs:ResMut<Assets<MaterialDef>>,mut meshs:ResMut<Assets<Mesh>>) {
    let root = {
        let mut root = commands.spawn();
        let t = Transform::default();
        root.insert(t);
        let camera = Camera::from_2d(Orthographic::default());
        root.insert(camera);

        let test = TestComponent {number: 0};
        root.insert(test);
        root.id()
    };

    
    create_elem(&mut commands, Vec3::new(2f32, 2f32, 2f32), root,&mut meshs);
    //create_elem(&mut  commands, Vec3::new(1f32, 1f32, 1f32), root);
    
    let test_md_string = std::fs::read_to_string("res/material/ui.md.clj").unwrap();
    let mut vm = EvalRT::new();
    let material_def = read_material_def(&mut vm, &test_md_string).unwrap();
    dbg!(&material_def);

    let id = HandleId::random::<MaterialDef>();
    mat_defs.set_untracked(id, material_def);
}

pub struct  TestComponent {
    number:u32
}

fn create_elem(commands:&mut Commands,pos:Vec3,parent:Entity,meshs:&mut Assets<Mesh>) -> Entity {
    let mut elem = commands.spawn();
    let mut t = Transform::default();
    t.local.position = pos;
    elem.insert(t);
    let mat = Material {order : RenderOrder::Transparent};
    elem.insert(mat);
    elem.insert(Parent(parent));

    let cube = Cube::new(2f32);
    let cube_mesh:Mesh = cube.into();

    let id = HandleId::random::<Mesh>();
    meshs.set_untracked(id, cube_mesh);
   
    elem.insert(id);
    elem.id()
}

fn on_update(mut commands:Commands,mut childrens:Query<(Entity,&mut TestComponent,&Camera)>,mut mat_def_events:EventReader<AssetEvent<MaterialDef>>) {
   for (e,mut t,_c) in childrens.iter_mut() {
       t.number += 1;
       if t.number > 100 {
          let mut e_cmd = commands.entity(e);
          e_cmd.remove::<Camera>();
       }
   }
   for ev in mat_def_events.iter() {
       dbg!(ev);
   }
}