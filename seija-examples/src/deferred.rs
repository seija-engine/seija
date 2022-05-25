use glam::{Vec3, Quat};
use seija_asset::{Assets, Handle};
use seija_core::{CoreStage, StartupStage, window::AppWindow, time::Time};
use seija_examples::{IExamples, add_render_mesh, load_material, load_texture};
use bevy_ecs::{prelude::{Commands, Entity, Query, Res, ResMut}, system::{IntoSystem,SystemParam}};
use seija_gltf::load_gltf;
use seija_pbr::lights::PBRLight;
use seija_render::{camera::camera::{Camera, Perspective}, material::{Material, MaterialStorage}, resource::{CubeMapBuilder, Mesh, shape::{Sphere, Cube, Quad}, Texture}};
use seija_skeleton3d::{Skeleton, AnimationSet, RuntimeSkeleton, Skin};
use seija_transform::Transform;

use crate::lib::{add_pbr_camera};
pub struct Deferred;

impl IExamples for Deferred {
    fn run(app:&mut seija_app::App) {
       app.add_system2(CoreStage::Startup,StartupStage::Startup, on_start.system());
       app.add_system(CoreStage::Update ,on_update.system());
    }
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

    {
            let point_light = PBRLight::directional(Vec3::new(1f32, 1f32, 1f32)  , 3000f32);
            let mut t = Transform::default();
            let r = Quat::from_euler(glam::EulerRot::XYZ  , 0f32, 0f32, 30f32.to_radians());
            t.local.rotation = r;
            let mut l = commands.spawn();
            l.insert(point_light);
            l.insert(t);
    }
    
    let coin_mesh = gltf_asset.first_mesh().unwrap();
    {
        let mut coin_entity = commands.spawn();
        let mut t = Transform::default();
        t.local.position = Vec3::new(0f32, 0f32, -1f32);
        coin_entity.insert(t);
        coin_entity.insert(coin_mesh.clone());
        let h_material = materials.create_material_with("DeferredPBR", |mat| {
            mat.texture_props.set("baseColor", gltf_asset.textures[0].clone());
            mat.texture_props.set("metallicRoughness", gltf_asset.textures[1].clone());
            mat.texture_props.set("normalTexture", gltf_asset.textures[2].clone());
        }).unwrap();
        coin_entity.insert(h_material);
    };
    
}


fn on_update(mut commands:Commands,time:Res<Time>,query:Query<(Entity,&Handle<RuntimeSkeleton>,&Handle<Mesh>)>) {
    //if time.frame() < 200 { return; }
    //for (e,_,_) in query.iter() {
    //    commands.entity(e).remove::<Handle<RuntimeSkeleton>>();
    //    commands.entity(e).remove::<Handle<Mesh>>();
    //}
}