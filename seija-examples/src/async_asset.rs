use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use glam::{Vec3, Vec4};
use seija_asset::Assets;
use seija_core::{window::AppWindow, CoreStage, StartupStage};
use seija_examples::{add_render_mesh, load_texture, IExamples};
use seija_gltf::{create_gltf, create_node, load_gltf};
use seija_render::wgpu;
use seija_render::{
    camera::camera::Camera,
    material::MaterialStorage,
    resource::{shape::Cube, Mesh, Texture},
};
use seija_transform::Transform;
pub struct AsyncAsset;

impl IExamples for AsyncAsset {
    fn run(app: &mut seija_app::App) {
        app.add_system2(CoreStage::Startup, StartupStage::Startup, on_start.system());
        app.add_system(CoreStage::Update, on_update.system());
    }
}

fn on_start(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut textures: ResMut<Assets<Texture>>,
    window: Res<AppWindow>,
    materials: Res<MaterialStorage>,
) {
    {
        let mesh = Cube::new(2f32);
        let hmesh = meshs.add(mesh.into());
        let hmat = materials
            .create_material_with("bplight", |mat| {
                //mat.props.set_float4("color", Vec4::new(0f32, 1f32, 0f32, 1f32), 0);
            })
            .unwrap();
        let mut t = Transform::default();
        t.local.scale = Vec3::new(1f32, 1f32, 1f32);
        t.local.position = Vec3::new(2f32, 0f32, -10f32);
        commands.spawn().insert(hmesh).insert(hmat).insert(t);
      };
    /*
       
       let h_texture = load_texture(&mut textures, "res/texture/b.jpg",Some(wgpu::TextureFormat::Rgba8UnormSrgb));
       {

         let mesh = Cube::new(2f32);
         let hmesh = meshs.add(mesh.into());
         let hmat = materials
             .create_material_with("puretexture", move|mat| {
                mat.texture_props.set("mainTexture", h_texture.clone());
                //mat.props.set_float4("color", Vec4::new(0.1f32, 0f32, 0f32, 1f32), 0);
             })
             .unwrap();
         let mut t = Transform::default();
         t.local.scale = Vec3::new(1f32, 1f32, 1f32);
         t.local.position = Vec3::new(-2f32, 0f32, -10f32);
         commands.spawn().insert(hmesh).insert(hmat).insert(t);
       };
    */
    /* 
    let gltf_asset = load_gltf(
        "res/gltf/Apple/apple.glb",
        &mut meshs,
        &mut textures,
    )
    .unwrap();
    let first_primtives = &gltf_asset.first_mesh().primitives[0];
    let gltf_mesh = first_primtives.mesh.clone();
    let gltf_texture = first_primtives
        .material
        .as_ref()
        .unwrap()
        .base_color_texture
        .as_ref()
        .unwrap()
        .clone();

    let mode_id = add_render_mesh(
        &mut commands,
        gltf_mesh,
        gltf_texture,
        "puretexture",
        Vec3::new(0f32, -0.05f32, -0.15f32),
        &materials,
    );*/
}

fn on_update(mut query: Query<(Entity, &Camera, &mut Transform)>) {}
