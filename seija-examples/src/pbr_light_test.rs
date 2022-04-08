use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, QuerySet, Res, ResMut};
use glam::{Vec3, Quat, Vec4};
use seija_asset::Assets;
use seija_core::{window::AppWindow, CoreStage, StartupStage};
use seija_examples::{IExamples};
use seija_pbr::PBRCameraInfo;
use seija_pbr::lights::PBRLight;
use seija_render::camera::camera::Perspective;
use seija_render::light::{Light};
use seija_render::resource::shape::{Sphere, Plane};
use seija_render::wgpu::{self};
use seija_render::{
    camera::camera::Camera,
    material::MaterialStorage,
    resource::{shape::Cube, Mesh, Texture},
};
use seija_transform::Transform;
pub struct PBRLightTest;

pub struct PingPongNumbers {
    pub light_pos:f32,
    pub light_pos_dir:f32,
}

impl Default for PingPongNumbers {
    fn default() -> Self {
        PingPongNumbers {light_pos:0f32,light_pos_dir:1f32 }
    }
}

impl IExamples for PBRLightTest {
    fn run(app: &mut seija_app::App) {
        app.world.insert_resource(PingPongNumbers::default());
        app.add_system2(CoreStage::Startup, StartupStage::Startup, on_start.system());
        app.add_system(CoreStage::Update, on_update.system());
    }
}

fn add_pbr_camera(window:&AppWindow,commands: &mut Commands) {
    let pbr_camera = PBRCameraInfo::default();
    let mut root = commands.spawn();
    let mut t = Transform::default();
    t.local.position = Vec3::new(0f32, 20f32, 70f32);
    t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ , -15f32 *  0.0174533f32, 0f32, 0f32); 
    root.insert(t);
    let mut per = Perspective::default();
    per.aspect_ratio = window.width() as f32 / window.height() as f32;
    let camera = Camera::from_3d(per);
    root.insert(camera);
    root.insert(pbr_camera);
}

fn on_start(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut textures: ResMut<Assets<Texture>>,
    window: Res<AppWindow>,
    materials: Res<MaterialStorage>,
) {
    add_pbr_camera(&window, &mut commands);
    {
        let point_light = PBRLight::point(Vec4::new(1f32, 1f32, 1f32, 1f32)  , 9000f32, 10f32);
        let mut t = Transform::default();
        let mut l = commands.spawn();
        l.insert(point_light);
        l.insert(t);
    }
}

fn on_update(mut query: QuerySet<(Query<(Entity, &mut Light, &mut Transform)>,Query<(Entity,&Camera,&mut Transform)>)>,mut numbers:ResMut<PingPongNumbers>) {
   
}
