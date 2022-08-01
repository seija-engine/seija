use bevy_ecs::system::{Commands, ResMut};
use glam::{Vec3, Quat, Vec4};
use seija_asset::Assets;
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::{init_core_app, add_pbr_camera, load_material};
use seija_pbr::lights::PBRLight;
use seija_render::{resource::{Mesh, shape::{Sphere, Cube, Plane}}, material::MaterialStorage, shadow::{ShadowLight, Shadow, ShadowCamera}};
use bevy_ecs::prelude::*;
use seija_transform::Transform;
pub fn main() {
    let mut app = init_core_app("shadow_render.clj");
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    //app.add_system(CoreStage::Update, on_update);
    app.run();
}

fn start(mut commands:Commands,window:Res<AppWindow>,mut meshs: ResMut<Assets<Mesh>>,materials: Res<MaterialStorage>) {
    let camera_pos = Vec3::new(0f32, 1.7f32, 3.71f32);
    let r = Quat::from_euler(glam::EulerRot::XYZ,
                                    -20f32.to_radians(), 
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
        let r = Quat::from_euler(glam::EulerRot::default()  , 90f32.to_radians(),  45f32.to_radians(), 0f32.to_radians());
        t.local.rotation = r;
        let mut l = commands.spawn();
        l.insert(light);
        l.insert(t);
        let mut shadow_light = ShadowLight::default();
        shadow_light.bias = 0.005f32;
        l.insert(shadow_light);
    }
        //sphere
        {
            let mesh =  Sphere::new(0.5f32);
            let hmesh = meshs.add(mesh.into());
            let hmat = materials.create_material_with("pbrColorShadow", |mat| {
                mat.props.set_f32("metallic",  0.3f32, 0);
                mat.props.set_f32("roughness", 0.7f32, 0);
                //mat.props.set_float4("color", Vec4::new(0f32, 0f32, 1f32, 1f32), 0)
            }).unwrap();
    
            let mut t = Transform::default();
            t.local.position = Vec3::new(-2f32, 0.5f32, 0f32);
            t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, 0f32.to_radians(), 0f32);
            let shadow = Shadow {cast_shadow:true,receive_shadow:true };
            commands.spawn().insert(hmesh).insert(hmat).insert(t).insert(shadow );
        };
        
        //Cube
        {
            let mesh =  Cube::new(1f32);
            let hmesh = meshs.add(mesh.into());
            let hmat = materials.create_material_with("pbrColorShadow", |mat| {
                mat.props.set_f32("metallic",  0.5f32, 0);
                mat.props.set_f32("roughness", 0.5f32, 0);
                //mat.props.set_float4("color", Vec4::new(0f32, 0f32, 1f32, 1f32), 0)
            }).unwrap();
    
            let mut t = Transform::default();
            t.local.scale = Vec3::new(1f32, 1f32, 1f32);
            t.local.position = Vec3::new(1f32, 0.5f32, 0f32);
            //t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, -40f32.to_radians(), 0f32);
            let shadow = Shadow {cast_shadow:true,receive_shadow:true };
            commands.spawn().insert(hmesh).insert(hmat).insert(t).insert(shadow );
        };
        //plane
        {
            
            let mesh =  Plane::new(100f32,10).into();
            let hmesh = meshs.add(mesh);
            let hmat = materials.create_material_with("pbrColorShadow", |mat| {
                mat.props.set_f32("metallic",  0.5f32, 0);
                mat.props.set_f32("roughness", 0.5f32, 0);
                mat.props.set_float4("color", Vec4::new(1f32, 1f32, 1f32, 1f32), 0)
            }).unwrap();
            let t = Transform::default();
            
           
            let shadow = Shadow {cast_shadow:true,receive_shadow:true };
            commands.spawn().insert(hmesh).insert(hmat).insert(t).insert(shadow );
        };
}