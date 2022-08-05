use bevy_ecs::system::{Commands, ResMut};
use glam::{Vec3, Quat};
use seija_asset::Assets;
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::{init_core_app, add_pbr_camera, load_material};
use seija_input::{event::{KeyboardInput, MouseButton},keycode::KeyCode, Input};
use seija_pbr::lights::PBRLight;
use seija_render::{resource::{Mesh, shape::{Cube, Sphere}}, material::MaterialStorage};
use bevy_ecs::prelude::*;
use seija_transform::Transform;
pub fn main() {
    let mut app = init_core_app("fxaa_render.clj");
    app.add_system2(CoreStage::Startup, StartupStage::PreStartup, start);
    app.add_system(CoreStage::Update, on_update);
    app.run();
}

fn start(mut commands:Commands,window:Res<AppWindow>,mut meshs: ResMut<Assets<Mesh>>,materials: Res<MaterialStorage>) {
    add_pbr_camera(&mut commands,&window,Vec3::new(0f32, 0f32, 2f32),Quat::IDENTITY,None);
    load_material("res/materials/pbrColor.mat.clj", &materials);
    //light
    {
        let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 62000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::default()  , 90f32.to_radians(),  45f32.to_radians(), 0f32.to_radians());
        t.local.rotation = r;
        let mut l = commands.spawn();
        l.insert(light);
        l.insert(t);
    }

    //Cube
    {
        let mesh =  Cube::new(1f32);
        let hmesh = meshs.add(mesh.into());
        let hmat = materials.create_material_with("pbrColor", |mat| {
            mat.props.set_f32("metallic",  0.5f32, 0);
            mat.props.set_f32("roughness", 0.5f32, 0);
            //mat.props.set_float4("color", Vec4::new(0f32, 0f32, 1f32, 1f32), 0)
        }).unwrap();

        let mut t = Transform::default();
        t.local.scale = Vec3::new(1f32, 1f32, 1f32);
        t.local.position = Vec3::new(0f32, 0f32, -0.5f32);
        t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, -31f32.to_radians(), 0f32);
       
        commands.spawn().insert(hmesh).insert(hmat).insert(t);
    };
}

fn on_update(input:Res<Input>) {
    if input.get_mouse_down(MouseButton::Left) {
        log::error!("left down");
    }

    if input.get_mouse_up(MouseButton::Left) {
        log::error!("left up");
    }
}