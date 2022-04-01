use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, QuerySet, Res, ResMut};
use glam::{Vec3, Vec4, Quat};
use seija_asset::Assets;
use seija_core::time::Time;
use seija_core::{window::AppWindow, CoreStage, StartupStage};
use seija_examples::{add_render_mesh, load_texture, IExamples};
use seija_gltf::{create_gltf, create_node, load_gltf};
use seija_render::light::{Light, LightEnv, LightType};
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

fn on_start(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut textures: ResMut<Assets<Texture>>,
    window: Res<AppWindow>,
    mut light_env:ResMut<LightEnv>,
    materials: Res<MaterialStorage>,
) {
    {
        light_env.set_ambient_color(Vec4::ZERO);
        
    };

    {
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::XYZ  , -15f32.to_radians(), 0f32, 0f32);
        t.local.rotation = r;
        let  light = Light::directional(Vec3::new(1f32, 1f32, 1f32), 0.5f32);
        commands.spawn().insert(light).insert(t);
        
    };

    {
        let mesh = Sphere::new(8f32);
        let hmesh = meshs.add(mesh.into());
        let hmat = materials.create_material_with("pbrColor", |mat| {}).unwrap();
        let mut t = Transform::default();
      
        t.local.position = Vec3::new(0f32, 6f32, 30f32);
        t.local.rotation = Quat::from_rotation_y(90f32 * 0.0174533f32);
        commands.spawn().insert(hmesh).insert(hmat).insert(t);
    };
   
}

fn on_update(mut query: QuerySet<(Query<(Entity, &mut Light, &mut Transform)>,Query<(Entity,&Camera,&mut Transform)>)>,mut numbers:ResMut<PingPongNumbers>) {
    let mut v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 3600) as f32;
    let half = v / 2f32;
    let num = v * 0.1f32 * 0.0174533f32;
    for (_,mut _light,mut t) in query.q0_mut().iter_mut() {
        let r = Quat::from_euler(glam::EulerRot::XYZ  , 0f32 , num , 0f32);
        t.local.rotation = r;
    }
}
