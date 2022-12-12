use bevy_ecs::system::{Commands,  CommandQueue};
use glam::{Vec3, Quat};
use seija_asset::Assets;
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::{init_core_app, add_pbr_camera, load_material};
use seija_pbr::lights::PBRLight;
use seija_render::{resource::{Mesh, shape::{Sphere}}, shadow::{ ShadowCamera}, material::Material};
use bevy_ecs::prelude::*;
use seija_transform::Transform;
pub fn main() {
    let mut app = init_core_app("bloom_render.clj",vec![]);
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start.exclusive_system());
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
    add_pbr_camera(&mut commands,&window,camera_pos,r,
        |e| {
            e.insert(ShadowCamera);
         },None,None,false);
    queue.apply(world);
    
    load_material("materials/pbrColor.mat.clj", world);
    load_material("materials/pbrColorShadow.mat.clj", world);
    
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
    
        //sphere
        {
            let mut meshs = world.get_resource_mut::<Assets<Mesh>>().unwrap();
            let mesh =  Sphere::new(0.5f32);
            let hmesh = meshs.add(mesh.into());

            let mut material = Material::from_world(world, "materials/pbrColor.mat.clj").unwrap();
            let mut materials = world.get_resource_mut::<Assets<Material>>().unwrap();
            material.props.set_f32("metallic",  0.3f32, 0);
            material.props.set_f32("roughness", 0.7f32, 0);
            let hmat = materials.add(material);
    
            let mut t = Transform::default();
            t.local.position = Vec3::new(0f32, 0f32, -1f32);
            t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, 0f32.to_radians(), 0f32);
            
            world.spawn().insert(hmesh).insert(hmat).insert(t);
        };
}