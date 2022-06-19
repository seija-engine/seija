use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use glam::{Quat, Vec3, Vec4};
use seija_asset::{Assets, Handle};
use seija_core::{window::AppWindow, CoreStage, StartupStage, time::Time};
use seija_examples::{load_texture, IExamples, add_pbr_camera, load_material};

use seija_pbr::lights::PBRLight;
use seija_render::{
    light::LightEnv,
    material::MaterialStorage,
    resource::{
        shape::{Cube, Sphere, Quad},
        Mesh, Texture,
    },
    wgpu, graph::nodes::{ShadowMapNode, ShadowLight},
};
use seija_transform::Transform;

pub struct ShadowTest;

impl IExamples for ShadowTest {
    fn run(app: &mut seija_app::App) {
        app.add_system2(CoreStage::Startup, StartupStage::Startup, on_start.system());
        app.add_system(CoreStage::Update, on_update.system());

        app.world.insert_resource(ShadowLight::new(Vec3::new(0f32,-0.5f32, -0.5f32)));
    }
}

fn on_start(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut textures: ResMut<Assets<Texture>>,
    window: Res<AppWindow>,
    materials: Res<MaterialStorage>,
) {
    load_material("res/materials/pbrColor.mat.clj", &materials);
    add_pbr_camera(&window, &mut commands);
    {
        let light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 12000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::XYZ  , 45f32.to_radians(), 0f32, 0f32.to_radians());
        t.local.rotation = r;
        let mut l = commands.spawn();
        l.insert(light);
        l.insert(t);
    }
    let mesh = Cube::new(2f32);
    let hmesh = meshs.add(mesh.into());
    let hmat = materials.create_material_with("pbrColor", |mat| {
            mat.props.set_f32("metallic",  0.5f32, 0);
            mat.props.set_f32("roughness", 0.5f32, 0);
            mat.props.set_float4("color", Vec4::new(1f32, 0f32, 1f32, 1f32), 0)
    }).unwrap();
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32, 0f32, 0f32);
    let mut cmds = commands.spawn();
    cmds.insert(hmesh).insert(hmat.clone()).insert(t);

    

    
}

fn on_update(
    mut light: ResMut<LightEnv>
    ,time:Res<Time>,
) {
}
