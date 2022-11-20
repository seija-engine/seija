use glam::{Vec3, Quat, Vec4};
use seija_asset::Assets;
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::{init_core_app, add_pbr_camera, load_material, update_camera_trans_system};
use seija_pbr::lights::PBRLight;
use seija_render::{resource::{Mesh, shape::{Sphere, Cube, Plane}},  shadow::{ShadowLight, Shadow, ShadowCamera}, material::Material};
use bevy_ecs::{prelude::*, system::CommandQueue};
use seija_transform::Transform;
pub fn main() {
    let mut app = init_core_app("FRPRender.clj");
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start.exclusive_system());
    app.add_system(CoreStage::Update, update_camera_trans_system);
    app.run();
}

fn start(world:&mut World) {
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, world);
    let window = world.get_resource::<AppWindow>().unwrap();
    let camera_pos = Vec3::new(0f32, 1.7f32, 3.71f32);
    let r = Quat::from_euler(glam::EulerRot::XYZ,
                                    -20f32.to_radians(), 
                                    0f32.to_radians(), 
                                    0f32.to_radians());
    add_pbr_camera(&mut commands,&window,camera_pos,r,
        |e| {
            e.insert(ShadowCamera );
         },None,None);
    load_material("materials/pbrColor.mat.clj", world);
    load_material("materials/pbrColorShadow.mat.clj", world);
    queue.apply(world);
    //light
    {
        let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 62000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::default()  , 90f32.to_radians(),  45f32.to_radians(), 0f32.to_radians());
        t.local.rotation = r;
        let mut l = world.spawn();
        l.insert(light);
        l.insert(t);
        let mut shadow_light = ShadowLight::default();
        shadow_light.bias = 0.005f32;
        l.insert(shadow_light);
    }
        
        //sphere
        {
            let mut meshs = world.get_resource_mut::<Assets<Mesh>>().unwrap();
            let mesh =  Sphere::new(0.5f32);
            let hmesh = meshs.add(mesh.into());
            let mut mat = Material::from_world(world, "materials/pbrColorShadow.mat.clj").unwrap();
            mat.props.set_f32("metallic",  0.3f32, 0);
            mat.props.set_f32("roughness", 0.7f32, 0);
            let mut mats = world.get_resource_mut::<Assets<Material>>().unwrap();
            let hmat = mats.add(mat);
    
            let mut t = Transform::default();
            t.local.position = Vec3::new(-2f32, 0.5f32, 0f32);
            t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, 0f32.to_radians(), 0f32);
            let shadow = Shadow {cast_shadow:true,receive_shadow:true };
            world.spawn().insert(hmesh).insert(hmat).insert(t).insert(shadow );
        };
        
        //Cube
        {
            let mut meshs = world.get_resource_mut::<Assets<Mesh>>().unwrap();
            let mesh =  Cube::new(1f32);
            let hmesh = meshs.add(mesh.into());
           
            let mut mat = Material::from_world(world, "materials/pbrColorShadow.mat.clj").unwrap();
            mat.props.set_f32("metallic",  0.5f32, 0);
            mat.props.set_f32("roughness", 0.5f32, 0);
            let mut mats = world.get_resource_mut::<Assets<Material>>().unwrap();
            let hmat = mats.add(mat);
    
            let mut t = Transform::default();
            t.local.scale = Vec3::new(1f32, 1f32, 1f32);
            t.local.position = Vec3::new(1f32, 0.5f32, 0f32);
            //t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, -40f32.to_radians(), 0f32);
            let shadow = Shadow {cast_shadow:true,receive_shadow:true };
            world.spawn().insert(hmesh).insert(hmat).insert(t).insert(shadow );
        };
        //plane
        {
            let mut meshs = world.get_resource_mut::<Assets<Mesh>>().unwrap();
            let mesh =  Plane::new(100f32,10).into();
            let hmesh = meshs.add(mesh);
            
            let mut mat = Material::from_world(world, "materials/pbrColorShadow.mat.clj").unwrap();
            mat.props.set_f32("metallic",  0.5f32, 0);
            mat.props.set_f32("roughness", 0.5f32, 0);
            mat.props.set_float4("color", Vec4::new(1f32, 1f32, 1f32, 1f32), 0);
            let mut mats = world.get_resource_mut::<Assets<Material>>().unwrap();
            let hmat = mats.add(mat);
            let t = Transform::default();
            
           
            let shadow = Shadow {cast_shadow:true,receive_shadow:true };
            world.spawn().insert(hmesh).insert(hmat).insert(t).insert(shadow );
        };
}