use std::path::Path;

use glam::{Vec3, Vec4, Quat};
use seija_asset::{Assets, Handle};
use seija_core::{CoreStage, StartupStage, bytes::FromBytes, window::AppWindow, time::Time};
use seija_examples::{IExamples, add_render_mesh, load_material};
use bevy_ecs::{prelude::{Commands, Entity, Query, Res, ResMut}, system::{IntoSystem,SystemParam}};
use seija_gltf::{create_gltf, load_gltf};
use seija_render::{camera::camera::{Camera, Perspective}, material::{Material, MaterialStorage}, resource::{CubeMapBuilder, Mesh, Texture}};
use seija_skeleton3d::{Skeleton, AnimationSet, RuntimeSkeleton, AnimationControl, Skin};
use seija_transform::{Transform, hierarchy::Parent, BuildChildren};
use crate::lib::{add_pbr_camera};
pub struct SampleGltf;

impl IExamples for SampleGltf {
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
    load_material("res/materials/pbrColor.mat.clj", &materials);
    load_material("res/materials/texture.mat.clj", &materials);
    let gltf_asset = load_gltf("res/gltf/Fox/glTF-Binary/Fox.glb",
                                   &mut meshs,
                                 &mut textures,
                                &mut skeletons,
                            &mut animations,
                                               &mut skins).unwrap();
    let h_skeleton = gltf_asset.skeleton.clone().unwrap();
    let count = skeletons.get(&h_skeleton.id).unwrap().num_joints();
    let mut animation_control = AnimationControl::new(count,
        h_skeleton,
     gltf_asset.anims.clone().unwrap(), &mut rtskeletons);
    animation_control.play_index(0);
    let clone_runtime_skeleton = animation_control.get_runtime_skeleton().clone();
    let fox_mesh_id = {
        let first_mesh = gltf_asset.first_mesh().unwrap();
        let mut fox = commands.spawn();
        fox.insert(clone_runtime_skeleton);
        let fox_mesh = fox.insert(Transform::default());
        fox_mesh.insert(first_mesh.primitives[0].mesh.clone());
        let h_material = materials.create_material_with("skinTexture", |mat| {
            mat.texture_props.set("mainTexture", gltf_asset.textures[0].clone());
            mat.props.set_float4("color", Vec4::new(0.6f32, 0.6f32, 0.6f32, 1f32), 0);
        }).unwrap();
        fox_mesh.insert(h_material);
        fox_mesh.insert(gltf_asset.skins.unwrap().clone());
        fox_mesh.id()
    };
    
    
    let mut root_t:Transform = Transform::default();
    root_t.local.position = Vec3::new(0f32, -80f32, -200f32);
    root_t.local.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0f32, 45f32.to_radians(), 0f32);
    let mut fox_root = commands.spawn();
    fox_root.insert(root_t);
    fox_root.insert(animation_control);
    fox_root.add_children(&vec![fox_mesh_id]);
   
}


fn on_update(mut commands:Commands,time:Res<Time>,query:Query<(Entity,&Handle<RuntimeSkeleton>,&Handle<Mesh>)>) {
    //if time.frame() < 200 { return; }
    //for (e,_,_) in query.iter() {
    //    commands.entity(e).remove::<Handle<RuntimeSkeleton>>();
    //    commands.entity(e).remove::<Handle<Mesh>>();
    //}
}