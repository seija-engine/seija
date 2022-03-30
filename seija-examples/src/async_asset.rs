use bevy_ecs::prelude::{Commands, Entity, IntoSystem, Query, Res, ResMut};
use glam::{Vec3, Vec4, Quat};
use seija_asset::Assets;
use seija_core::{window::AppWindow, CoreStage, StartupStage};
use seija_examples::{add_render_mesh, load_texture, IExamples};
use seija_gltf::{create_gltf, create_node, load_gltf};
use seija_render::light::Light;
use seija_render::resource::shape::Sphere;
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
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::XYZ  , 0f32, 45f32, 0f32);
        t.local.rotation = r;
        let mut light = Light::spot(Vec3::new(1f32, 1f32, 1f32), 1f32,10f32,30f32);
        light.color = Vec3::new(1f32, 1f32, 1f32);
        light.intensity = 0.5f32;
        commands.spawn().insert(light).insert(t);
        
    };
    
    {
        let mesh = Sphere::new(2f32);
        let hmesh = meshs.add(mesh.into());
        let hmat = materials.create_material_with("bplight", |mat| {
                                                                        //mat.props.set_float4("color", Vec4::new(0f32, 1f32, 0f32, 1f32), 0);
                                                                }).unwrap();
        let mut t = Transform::default();
        t.local.scale = Vec3::new(1f32, 1f32, 1f32);
        t.local.position = Vec3::new(0f32, 0f32, -15f32);
        commands.spawn().insert(hmesh).insert(hmat).insert(t);
      };
     /**/
    /* 
    let asset = load_gltf("res/gltf/barrel/barrel_03_1k.gltf",&mut meshs,&mut textures).unwrap();
   create_gltf(
  Vec3::new(0f32, -0.5f32, -2f32),
      &asset, &mut commands,&|gltf_material| {
      if let Some(texture) = gltf_material.base_color_texture.as_ref() {
         materials.create_material_with("puretexture",|mat| {
            mat.texture_props.set("mainTexture", texture.clone());
         })
      } else {
         materials.create_material_with("puretexture",|mat| {
            mat.props.set_float4("color", Vec4::from(gltf_material.base_color), 0);
         }) 
      }
   });*/
   
}

fn on_update(mut query: Query<(Entity, &mut Light, &mut Transform)>) {
    let v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 3600) as f32;
    let r = v * 0.1f32 * 0.0174533f32;
    for (_,_,mut t) in query.iter_mut() {
        let r = Quat::from_euler(glam::EulerRot::XYZ  , 0f32, r, 0f32);
        t.local.rotation = r;
    }
}
