use glam::{Vec3, Quat};
use seija_asset::{Assets, Handle};
use seija_core::{CoreStage, StartupStage, window::AppWindow, time::Time};
use seija_examples::{IExamples, add_render_mesh, load_material, load_texture};
use bevy_ecs::{prelude::{Commands, Entity, Query, Res, ResMut}, system::{IntoSystem,SystemParam}};
use seija_gltf::load_gltf;
use seija_pbr::lights::{PBRLight, PBRLightType};
use seija_render::{camera::camera::{Camera, Perspective}, material::{Material, MaterialStorage}, resource::{CubeMapBuilder, Mesh, shape::{Sphere, Cube, Quad, Plane}, Texture}};
use seija_skeleton3d::{Skeleton, AnimationSet, RuntimeSkeleton, Skin};
use seija_transform::Transform;

use crate::lib::{add_pbr_camera};
pub struct Deferred;

impl IExamples for Deferred {
    fn run(app:&mut seija_app::App) {
       app.world.insert_resource(AnimationNumbers::default());

       app.add_system2(CoreStage::Startup,StartupStage::Startup, on_start.system());
       app.add_system(CoreStage::Update ,on_update.system());
    }
}

#[derive(Default)]
pub struct AnimationNumbers {
    point_size:f32
}




fn on_start(mut commands:Commands,
            mut meshs:ResMut<Assets<Mesh>>,
            mut textures:ResMut<Assets<Texture>>,
            mut skeletons:ResMut<Assets<Skeleton>>,
            mut animations:ResMut<Assets<AnimationSet>>,
            mut rtskeletons:ResMut<Assets<RuntimeSkeleton>>,
            mut skins:ResMut<Assets<Skin>>,
            window:Res<AppWindow>,
            materials:Res<MaterialStorage>) {
    add_pbr_camera(&window, &mut commands);
    load_material("res/materials/deferredPBR.mat.clj", &materials);
    
    //let h_texture = load_texture(&mut textures, "res/texture/b.jpg",None);
    let gltf_asset = load_gltf("res/gltf/coin/scene.gltf",
                                   &mut meshs,
                                 &mut textures,
                                &mut skeletons,
                            &mut animations,
                                              &mut skins).unwrap();
   let colors = vec![Vec3::new(1f32,0f32,0f32),
                                Vec3::new(1f32,122f32 / 255f32,0f32),
                                Vec3::new(1f32,1f32,0f32),
                                Vec3::new(0f32,1f32,0f32),
                                Vec3::new(1f32,1f32,1f32),
                                Vec3::new(0f32,0f32,1f32),
                                Vec3::new(1f32,0f32,1f32)];
   let mut index = 0;
   for x in 0..8 {
       for y in 0..7 {
        let c = colors[index];
        let point_light = PBRLight::point(c, 90000f32,10f32);
        let mut t = Transform::default();
        t.local.position = Vec3::new(-50f32 + x as f32 * (100f32 / 8f32), -71f32, -100f32 + (y as f32 * (100f32 / 7f32)));
        let mut l = commands.spawn();
        l.insert(point_light);
        l.insert(t);

        index = index + 1;
        if index >= colors.len() {
            index = 0;
        }
       }
   }
 
    /* 
    {
        let point_light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 3000f32);
        let mut t = Transform::default();
        let r = Quat::from_euler(glam::EulerRot::XYZ  , 0f32, 0f32, 30f32.to_radians());
        t.local.rotation = r;
        let mut l = commands.spawn();
        l.insert(point_light);
        l.insert(t);
}*/
    {
        let h_texture = load_texture(&mut textures, "res/texture/WoodFloor043_1K_Color.jpg");
        let h_roughness = load_texture(&mut textures, "res/texture/tield/Material_metallicRoughness.png");
        let h_normal = load_texture(&mut textures, "res/texture/WoodFloor043_1K_Normal.jpg");

        let mesh = Quad::new(100f32);
        let hmesh = meshs.add(mesh.into());
        let hmat = materials.create_material_with("DeferredPBR", |mat| {
            mat.texture_props.set("baseColor", h_texture.clone());
            mat.texture_props.set("metallicRoughness", h_roughness.clone());
            mat.texture_props.set("normalTexture", h_normal.clone());
            mat.props.set_f32("metallicFactor", 1f32, 0);
            mat.props.set_f32("roughnessFactor", 0f32, 0);
        }).unwrap();
        let mut t = Transform::default();
        t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ  , -90f32.to_radians(), 0f32, 0f32);
        t.local.position = Vec3::new(0f32, -80f32, -50f32);
        commands.spawn().insert(hmesh).insert(hmat).insert(t);
    };

     
    let coin_mesh = gltf_asset.first_mesh().unwrap();
    {
        let mut coin_entity = commands.spawn();
        let mut t = Transform::default();
        t.local.position = Vec3::new(0f32, -70f32, -15f32);
        t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ  , -90f32.to_radians(), 0f32, 0f32);
        coin_entity.insert(t);
        coin_entity.insert(coin_mesh.clone());
        let h_material = materials.create_material_with("DeferredPBR", |mat| {
            mat.texture_props.set("baseColor", gltf_asset.textures[0].clone());
            mat.texture_props.set("metallicRoughness", gltf_asset.textures[1].clone());
            mat.texture_props.set("normalTexture", gltf_asset.textures[2].clone());
        }).unwrap();
        coin_entity.insert(h_material);
    };/**/
    
}


fn on_update(mut commands:Commands,time:Res<Time>,mut numbers:ResMut<AnimationNumbers>,mut query_camera:Query<(Entity,&mut Transform,&mut PBRLight)>) {
    let speed:u128 = 3;
    let v:f32 = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % (36000 * speed)) as f32;
    let r = v * 0.01f32 * 0.0174533f32  * speed as f32;
   

    for (e,mut t,mut light) in query_camera.iter_mut() {
        t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ  , -r, 0f32, 0f32);
        
        //log::error!("update :{:?}",t.local.rotation);
     }
}