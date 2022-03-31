use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, QuerySet, Res, ResMut};
use glam::{Vec3, Vec4, Quat};
use seija_asset::Assets;
use seija_core::time::Time;
use seija_core::{window::AppWindow, CoreStage, StartupStage};
use seija_examples::{add_render_mesh, load_texture, IExamples};
use seija_gltf::{create_gltf, create_node, load_gltf};
use seija_render::light::{Light, LightEnv};
use seija_render::resource::shape::{Sphere, Plane};
use seija_render::wgpu::{self};
use seija_render::{
    camera::camera::Camera,
    material::MaterialStorage,
    resource::{shape::Cube, Mesh, Texture},
};
use seija_transform::Transform;
pub struct LightTest;

pub struct PingPongNumbers {
    pub light_pos:f32,
    pub light_pos_dir:f32,
}

impl Default for PingPongNumbers {
    fn default() -> Self {
        PingPongNumbers {light_pos:0f32,light_pos_dir:1f32 }
    }
}

impl IExamples for LightTest {
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
        let r = Quat::from_euler(glam::EulerRot::XYZ  , 0f32, 45f32 * 0.0174533f32, 0f32);
        t.local.rotation = r;
        let mut light = Light::point(Vec3::new(1f32, 1f32, 1f32), 1f32,5f32);
        light.color = Vec3::new(1f32, 1f32, 1f32);
        light.intensity = 0.2f32;
        commands.spawn().insert(light).insert(t);
        
    };
    
    {
        let mesh = Sphere::new(8f32);
        let hmesh = meshs.add(mesh.into());
        let hmat = materials.create_material_with("bplight", |mat| {}).unwrap();
        let mut t = Transform::default();
      
        t.local.position = Vec3::new(20f32, 4f32, 30f32);
        t.local.rotation = Quat::from_rotation_y(90f32 * 0.0174533f32);
        commands.spawn().insert(hmesh).insert(hmat).insert(t);
    };

    {
        let mesh =  Cube::new(8f32);
        let hmesh = meshs.add(mesh.into());
        let hmat = materials.create_material_with("bplight", |mat| {}).unwrap();
        let mut t = Transform::default();
      
        t.local.position = Vec3::new(-20f32, 4f32, 30f32);
        t.local.rotation = Quat::from_rotation_y(90f32 * 0.0174533f32);
        commands.spawn().insert(hmesh).insert(hmat).insert(t);
    };

    {
        let mesh = Plane::new(100f32,10);
        let hmesh = meshs.add(mesh.into());
        let hmat = materials.create_material_with("bplight", |mat| {}).unwrap();
        let mut t = Transform::default();
       
        t.local.position = Vec3::new(-50f32, -1f32, -50f32);
        commands.spawn().insert(hmesh).insert(hmat).insert(t);
    };
     
    
   
}

fn on_update(mut query: QuerySet<(Query<(Entity, &mut Light, &mut Transform)>,Query<(Entity,&Camera,&mut Transform)>)>,mut numbers:ResMut<PingPongNumbers>) {
    let mut v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 36000) as f32;
    let half = v / 2f32;
    let num = half * 0.01f32 * 0.0174533f32;
    if numbers.light_pos > 70f32 {
        numbers.light_pos_dir = -1f32;
    }
    if numbers.light_pos < -50f32 {
        numbers.light_pos_dir = 1f32;
    }
    numbers.light_pos += numbers.light_pos_dir * 0.1f32;
    let new_pos = Vec3::new(0f32, 0f32, numbers.light_pos);

    for (_,mut light,mut t) in query.q0_mut().iter_mut() {
        let r = Quat::from_euler(glam::EulerRot::XYZ  , -num , 0f32 , 0f32);
        t.local.rotation = r;
        t.local.position = new_pos;
    }
    
}
