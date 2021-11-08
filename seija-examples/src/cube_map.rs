use std::path::Path;

use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use glam::{Quat, Vec3, Vec4};
use seija_asset::{Asset, Assets, Handle, HandleId};
use seija_core::{CoreStage, StartupStage, window::AppWindow};
use seija_examples::IExamples;
use seija_gltf::{create_gltf, load_gltf};
use seija_render::{camera::camera::Camera, material::MaterialStorage, resource::{CubeMapBuilder, Mesh, Texture, shape::{Cube, SkyBox}}};
use seija_transform::Transform;

pub struct CubeMapTest;

impl IExamples for CubeMapTest {
    fn run(app:&mut seija_app::App) {
       app.add_system2(CoreStage::Startup,StartupStage::Startup, on_start.system());
       app.add_system(CoreStage::Update, on_update.system());
    }
}

fn on_start(mut commands:Commands,
    mut meshs:ResMut<Assets <Mesh>>,
    mut textures:ResMut<Assets<Texture>>,
    window:Res<AppWindow>,
    materials:Res<MaterialStorage>) {

    let p = Path::new("res/texture/cubemap/Yokohama3/");
    let cube_builder = CubeMapBuilder::new();
    let texture = cube_builder.with_left_path(p.join("posx.jpg")).unwrap()
                .with_right_path(p.join("negx.jpg")).unwrap()
                .with_top_path(p.join("posy.jpg")).unwrap()
                .with_bottom_path(p.join("negy.jpg")).unwrap()
                .with_back_path(p.join("posz.jpg")).unwrap()
                .with_front_path(p.join("negz.jpg")).unwrap().build();
    let texture_handle = HandleId::random::<Texture>();
    textures.set_untracked(texture_handle,texture.unwrap());

    let sky:Mesh = SkyBox.into();
    let cube_mesh = meshs.add(sky);
    let t = Transform::default();
    let mat_handle = materials.create_material_with("sky-box", |mat| {
        mat.texture_props.set("mainTexture", Handle::weak(texture_handle));
    }).unwrap();
    commands.spawn().insert(t).insert(cube_mesh).insert(mat_handle);
 
}

fn on_update(mut query:Query<(Entity,&Camera,&mut Transform)>) {
    let v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 36000) as f32;
    for (e,m,mut t) in query.iter_mut() {
        let r = v * 0.01f32 * 0.0174533f32;
        t.local.position = Vec3::new(400f32, 0f32, 0f32);
        t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ  , 0f32, r, 0f32);
    }
 }