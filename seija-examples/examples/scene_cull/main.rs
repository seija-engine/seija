use bevy_ecs::system::{Commands, CommandQueue};
use glam::{Vec3, Quat};
use seija_asset::{Assets, AssetServer, Handle};
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::{init_core_app, add_pbr_camera, load_material, update_camera_trans_system};

use seija_pbr::lights::{PBRLight, PBRGlobalAmbient};
use seija_render::{resource::{Mesh}, material::Material};
use bevy_ecs::prelude::*;
use seija_transform::Transform;
pub fn main() {
    let mut app = init_core_app("model_render.clj");
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start.exclusive_system());
    app.add_system(CoreStage::Update, update_camera_trans_system);
    app.run();
}

fn start(world:&mut World) {
    let camera_pos = Vec3::new(0f32, 0f32, 2f32);
    let r = Quat::from_euler(glam::EulerRot::XYZ,
                                    0f32.to_radians(), 
                                    0f32.to_radians(), 
                                    0f32.to_radians());
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, world);
    let window = world.get_resource::<AppWindow>().unwrap();
    add_pbr_camera(&mut commands,&window,camera_pos,r,None, Some(150f32),Some(1));
    queue.apply(world);
    
    load_material("materials/pbrColor.mat.clj", world);
    load_material("materials/pbrColorShadow.mat.clj", world);
    world.init_resource::<PBRGlobalAmbient>();
    //light
    {
        let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 62000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::default()  , 180f32.to_radians(),  0f32.to_radians(), 0f32.to_radians());
        t.local.rotation = r;
        let mut l = world.spawn();
        l.insert(light);
        l.insert(t);
    }
    
    create_many_cubes(world);
}


fn create_many_cubes(world:&mut World) {
    let mat = Material::from_world(world, "materials/pbrColor.mat.clj").unwrap();
    let mut mats = world.get_resource_mut::<Assets<Material>>().unwrap();
    let hmat = mats.add(mat);
    let hmesh:Handle<Mesh> = world.get_resource::<AssetServer>().unwrap().get_asset("mesh:cube").unwrap().make_handle().typed();
    
    for radius in 1..100 {
        for angle in 0..36 {
            let mut vec = Vec3::new(0f32, 0f32, -1f32);
            vec = vec * radius as f32 * 5f32;
            vec.y = 5f32;
            vec = Quat::from_euler(Default::default(), (angle as f32 * 10f32).to_radians(), 0f32, 0f32) * vec;
            let mut t = Transform::default();
            t.local.position = vec;
            world.spawn().insert(hmat.clone()).insert(hmesh.clone()).insert(t);
        }
    }
}