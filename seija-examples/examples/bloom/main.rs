use bevy_ecs::system::{Commands, ResMut};
use glam::{Vec3, Quat};
use seija_asset::Assets;
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::{init_core_app, add_pbr_camera, load_material};
use seija_pbr::lights::PBRLight;
use seija_render::{resource::{Mesh, shape::{Sphere}}, material::MaterialStorage, shadow::{ Shadow, ShadowCamera}};
use bevy_ecs::prelude::*;
use seija_transform::Transform;
pub fn main() {
    let mut app = init_core_app("bloom_render.clj");
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.run();
}

fn start(mut commands:Commands,window:Res<AppWindow>,mut meshs: ResMut<Assets<Mesh>>,materials: Res<MaterialStorage>) {
    let camera_pos = Vec3::new(0f32, 0f32, 2f32);
    let r = Quat::from_euler(glam::EulerRot::XYZ,
                                    0f32.to_radians(), 
                                    0f32.to_radians(), 
                                    0f32.to_radians());
    add_pbr_camera(&mut commands,&window,camera_pos,r,
                 Some(|e| {
                    e.insert(ShadowCamera );
                 }));
    load_material("res/materials/pbrColor.mat.clj", &materials);
    load_material("res/materials/pbrColorShadow.mat.clj", &materials);
    //light
    {
        let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 62000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::default()  , 180f32.to_radians(),  0f32.to_radians(), 0f32.to_radians());
        t.local.rotation = r;
        let mut l = commands.spawn();
        l.insert(light);
        l.insert(t);
    }
        //sphere
        {
            let mesh =  Sphere::new(0.5f32);
            let hmesh = meshs.add(mesh.into());
            let hmat = materials.create_material_with("pbrColor", |mat| {
                mat.props.set_f32("metallic",  0.3f32, 0);
                mat.props.set_f32("roughness", 0.7f32, 0);
                //mat.props.set_float4("color", Vec4::new(0f32, 0f32, 1f32, 1f32), 0)
            }).unwrap();
    
            let mut t = Transform::default();
            t.local.position = Vec3::new(0f32, 0f32, -1f32);
            t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, 0f32.to_radians(), 0f32);
            let shadow = Shadow {cast_shadow:true,receive_shadow:true };
            commands.spawn().insert(hmesh).insert(hmat).insert(t).insert(shadow );
        };
        
       
}