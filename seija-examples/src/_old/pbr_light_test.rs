use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, QuerySet, Res, ResMut};
use glam::{Vec3, Quat, Vec4};
use seija_asset::Assets;
use seija_core::{window::AppWindow, CoreStage, StartupStage};
use seija_examples::{IExamples, load_material};
use seija_pbr::PBRCameraInfo;
use seija_pbr::lights::PBRLight;
use seija_render::camera::camera::Perspective;
use seija_render::light::{Light};
use seija_render::resource::Texture;
use seija_render::resource::shape::{Sphere, Plane};
use seija_render::wgpu::{self};
use seija_render::{
    camera::camera::Camera,
    material::MaterialStorage,
    resource::{Mesh},
};
use seija_transform::Transform;

use crate::lib::add_pbr_camera;
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
        let point_light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 9000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::XYZ  , 0f32, 0f32, 45f32.to_radians());
        t.local.rotation = r;
        let mut l = commands.spawn();
        l.insert(point_light);
        l.insert(t);
    }
    for x in 0..5 {
        for y in 0..5 {
        {
            let mesh = Sphere::new(2f32);
            let hmesh = meshs.add(mesh.into());
            let hmat = materials.create_material_with("pbrColor", |mat| {
                mat.props.set_f32("metallic",  (1f32 / 5f32)  * x as f32, 0);
                mat.props.set_f32("roughness", (1f32 / 5f32) * y as f32, 0);
            }).unwrap();
            let mut t = Transform::default();
          
            t.local.position = Vec3::new(x as f32 * 5f32 - 10f32, y as f32 * 5f32, 30f32);
            t.local.rotation = Quat::from_rotation_y(90f32 * 0.0174533f32);
            commands.spawn().insert(hmesh).insert(hmat).insert(t);
        };
      }
    }
    
}

fn on_update(mut query: QuerySet<(Query<(Entity, &mut PBRLight, &mut Transform)>,Query<(Entity,&Camera,&mut Transform)>)>,mut numbers:ResMut<PingPongNumbers>) {
    let mut v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 3600) as f32;
    let num = v * 0.1f32 * 0.0174533f32;
    for (_,mut light,mut t) in query.q0_mut().iter_mut() {
        let r = Quat::from_euler(glam::EulerRot::XYZ  , 0f32 , -num , 0f32);
        t.local.rotation = r;
    }
}
