use std::path::Path;

use glam::{Vec3, Vec4};
use seija_asset::{Assets, Handle};
use seija_core::{CoreStage, StartupStage, bytes::FromBytes, window::AppWindow};
use seija_examples::{IExamples, add_render_mesh, load_material};
use bevy_ecs::{prelude::{Commands, Entity, Query, Res, ResMut}, system::{IntoSystem,SystemParam}};
use seija_gltf::{create_gltf, load_gltf};
use seija_render::{camera::camera::{Camera, Perspective}, material::{Material, MaterialStorage}, resource::{CubeMapBuilder, Mesh, Texture}};
use seija_skeleton3d::{Skeleton, AnimationSet};
use seija_transform::{Transform, hierarchy::Parent};
use crate::lib::{add_camera_3d};
pub struct SampleGltf;

impl IExamples for SampleGltf {
    fn run(app:&mut seija_app::App) {
       app.add_system2(CoreStage::Startup,StartupStage::Startup, on_start.system());
    }
}





fn on_start(mut commands:Commands,
            mut meshs:ResMut<Assets<Mesh>>,
            mut textures:ResMut<Assets<Texture>>,
            mut skeletons:ResMut<Assets<Skeleton>>,
            mut animations:ResMut<Assets<AnimationSet>>,
            window:Res<AppWindow>,
            materials:Res<MaterialStorage>) {
    
    let gltf_asset = load_gltf("res/gltf/Fox/glTF-Binary/Fox.glb",
                                   &mut meshs,
                                 &mut textures,
                                &mut skeletons,
                            &mut animations).unwrap();
    

   
   
}
