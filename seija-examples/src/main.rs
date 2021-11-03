use std::{cell::RefMut, io::Read, ops::Sub};

use lite_clojure_eval::EvalRT;
use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use glam::{Quat, Vec3, Vec4};
use seija_app::App;
use seija_asset::{AssetEvent, AssetModule, Assets, Handle, HandleId};
use seija_core::{CoreModule, CoreStage, StartupStage, event::EventReader};
use seija_gltf::{asset::GltfAsset, load_gltf};
use seija_render::{RenderModule, camera::{camera::{Camera, Orthographic, Perspective}}, material::{Material, MaterialStorage, RenderOrder, read_material_def}, resource::{Mesh, Texture, shape::Cube}};
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
    number:i32,
    asset:GltfAsset
}

fn on_start_up(mut commands:Commands,mut meshs:ResMut<Assets<Mesh>>,storage:Res<MaterialStorage>,mut textures:ResMut<Assets<Texture>>) {
    let gltf_asset = load_gltf("res/gltf/Fox/glTF/Fox.gltf", &mut meshs).unwrap();
    let gltf_mesh = gltf_asset.meshs[0].primitives[0].mesh.clone_weak();
    let root = {
        let mut root = commands.spawn();
        let t = Transform::default();
        root.insert(t);
        let mut per = Perspective::default();
        per.aspect_ratio = 640f32 / 480f32;
        let camera = Camera::from_3d(per);
        root.insert(camera);

        
        let test = RootComponent {number: 0,asset:gltf_asset};
        root.insert(test);
        root.id()
    };
    
    let bytes = std::fs::read("res/gltf/Fox/glTF/Texture.png").unwrap();
    let tex = Texture::from_bytes(&bytes).unwrap();
    let wood_texture = textures.add(tex);

    let bytes = std::fs::read("res/gltf/Fox/glTF/Texture.png").unwrap();
    let tex = Texture::from_bytes(&bytes).unwrap();
    let b_texture = textures.add(tex);

    let texture_comp = TestComp { fst:wood_texture,snd:b_texture,mesh: gltf_mesh};

    println!("texture load success");

    let test_md_string = std::fs::read_to_string("res/material/model/model.mat.clj").unwrap();
    let mut vm = EvalRT::new();
    let material_def = read_material_def(&mut vm, &test_md_string).unwrap();
  
    storage.add_def(material_def);
  
    //create_elem(&mut commands, Vec3::new(8f32, 0f32, -20f32), 
    //     root,&mut meshs,&storage,Vec4::new(1f32, 0f32, 0f32, 1f32),wood_texture.clone_weak());

    create_elem(&mut commands, Vec3::new(0f32, -10f32, -250f32), root,&mut meshs,&storage,Vec4::new(1f32, 1f32, 1f32, 1f32),texture_comp);
}


struct TestComp {
    fst:Handle<Texture>,
    snd:Handle<Texture>,
    mesh:Handle<Mesh>
}


fn create_elem(commands:&mut Commands,pos:Vec3,parent:Entity,meshs:&mut Assets<Mesh>,
               storage:&Res<MaterialStorage>,color:Vec4,test_comp:TestComp) -> Entity {
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
    let mut mats = storage.mateials.write();
    let mat = mats.get_mut(&material.id).unwrap();
    mat.props.set_float4("color", color, 0);
    mat.texture_props.set("mainTexture", test_comp.fst.clone_weak());
    elem.insert(test_comp.mesh.clone_weak());
    elem.insert(material);
    elem.insert(test_comp);
    elem.id()
}

fn on_update(mats:ResMut<MaterialStorage>,mut renders:Query<(Entity,&mut Transform,&Handle<Mesh>,&Handle<Material>,&TestComp)>) {
    for (_e,mut t,_,mat_handle,test_comp) in renders.iter_mut() {
        let v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 3600) as f32;
       
        let mut mat_map = mats.mateials.write();
        let mat_mut = mat_map.get_mut(&mat_handle.id).unwrap();

        

        //if v > 1800f32 {
        //   mat_mut.texture_props.set("mainTexture", test_comp.fst.clone_weak());
        //} else {
        //   mat_mut.texture_props.set("mainTexture", test_comp.snd.clone_weak());
        //}

        t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 30f32 * 0.0174533f32, v * 0.1f32 * 0.0174533f32, 0f32);
        
       
       
    }
}