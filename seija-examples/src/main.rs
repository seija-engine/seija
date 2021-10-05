use lite_clojure_eval::EvalRT;
use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use glam::Vec3;
use seija_app::App;
use seija_asset::{AssetEvent, AssetModule, Assets, Handle, HandleId};
use seija_core::{CoreModule, CoreStage, StartupStage, event::EventReader};
use seija_render::{RenderModule, camera::{camera::{Camera, Orthographic}}, material::{Material, MaterialDef, MaterialStorage, RenderOrder, read_material_def}, resource::{shape::Cube,Mesh}};
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

pub struct  RootComponent {
    number:i32
}

fn on_start_up(mut commands:Commands,mat_def_center:Res<MaterialStorage>,mut meshs:ResMut<Assets<Mesh>>) {
    let root = {
        let mut root = commands.spawn();
        let t = Transform::default();
        root.insert(t);
        let camera = Camera::from_2d(Orthographic::default());
        root.insert(camera);

        
        let test = RootComponent {number: 0};
        root.insert(test);
        root.id()
    };
    
    let test_md_string = std::fs::read_to_string("res/material/ui.md.clj").unwrap();
    let mut vm = EvalRT::new();
    let material_def = read_material_def(&mut vm, &test_md_string).unwrap();
    let order = material_def.order;
    mat_def_center.add_def(material_def);
  
    create_elem(&mut commands, Vec3::new(2f32, 2f32, 2f32), root,&mut meshs,mat_def_center,order);
}





fn create_elem(commands:&mut Commands,pos:Vec3,parent:Entity,meshs:&mut Assets<Mesh>,mat_def_center:Res<MaterialStorage>,order: RenderOrder) -> Entity {
    let mut elem = commands.spawn();
    let mut t = Transform::default();
    t.local.position = pos;
    elem.insert(t);
    elem.insert(Parent(parent));

    let cube = Cube::new(2f32);
    let cube_mesh:Mesh = cube.into();   
    let cube_mesh_handle = meshs.add( cube_mesh);
    
    let material = mat_def_center.create_material("ui-color").unwrap();
   
    elem.insert(cube_mesh_handle);
    elem.insert(material);
    elem.id()
}

fn on_update(mut commands:Commands,mut childrens:Query<(Entity,&mut RootComponent,&Camera)>,mut mat_def_events:EventReader<AssetEvent<MaterialDef>>) {
   for (e,mut t,_c) in childrens.iter_mut() {
       t.number += 1;
       if t.number > 100 {
          let mut e_cmd = commands.entity(e);
          e_cmd.remove::<Camera>();
          e_cmd.remove::<RootComponent>();
       }
   }
   for ev in mat_def_events.iter() {
     
   }
}