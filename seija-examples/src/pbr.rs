use glam::{Quat, Vec3};
use seija_examples::{IExamples, load_texture};
use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use seija_asset::{Assets, Handle};
use seija_core::{CoreStage, StartupStage, window::AppWindow};

use seija_render::{light::LightEnv, material::MaterialStorage, resource::{Mesh, Texture, shape::Sphere}};
use seija_transform::Transform;

pub struct PbrTest;

impl IExamples for PbrTest {
    fn run(app:&mut seija_app::App) {
       app.add_system2(CoreStage::Startup,StartupStage::Startup, on_start.system());
       app.add_system(CoreStage::Update, on_update.system());
    }
}

fn on_start(mut commands:Commands,mut meshs:ResMut<Assets <Mesh>>,mut textures:ResMut<Assets<Texture>>,window:Res<AppWindow>,materials:Res<MaterialStorage>) {
    create_pbr_sphere(&mut commands,&mut textures,&mut meshs,materials);
}

fn create_pbr_sphere(commands:&mut Commands,textures:&mut Assets<Texture>,meshs:&mut Assets<Mesh>,materials:Res<MaterialStorage>) {
    let h_texture = load_texture(textures, "res/texture/WoodFloor043_1K_Color.jpg");
    let h_roughness = load_texture(textures, "res/texture/WoodFloor043_1K_Roughness.jpg");
    let h_normal = load_texture(textures, "res/texture/WoodFloor043_1K_Normal.jpg");
    let mesh:Mesh = Sphere::new(2f32).into();
    let h_mesh = meshs.add(mesh);
    let h_material = materials.create_material_with("pbr", |mat| {
        mat.texture_props.set("baseColor", h_texture.clone());
        mat.texture_props.set("roughness", h_roughness.clone());
        mat.texture_props.set("normal", h_normal.clone());
    });
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32, 0f32, -10f32);
    commands.spawn()
           .insert(h_mesh)
           .insert(h_material.unwrap())
           .insert(t);
}

fn on_update(light:ResMut<LightEnv>,mut query:Query<(Entity,&Handle<Mesh>,&mut Transform)>) {
    let v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 36000) as f32;
    let r = v * 0.01f32 * 0.0174533f32;
    
    for (_,_,mut t) in query.iter_mut() {
        t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ  , 0f32, r, r)
        
    }
}