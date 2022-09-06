use std::collections::HashMap;

use bevy_ecs::{system::{Res, Commands, Query, ResMut}, prelude::Entity};
use seija_asset::{Handle, Assets};
use seija_core::info::EInfo;
use seija_geometry::volume::AABB3;
use seija_transform::Transform;

use crate::{scene::SceneEnv, material::Material, resource::Mesh};

use super::scene_octree::{SceneOctree, NodeId};

pub struct SceneAABB {

}

pub struct SceneOctreeMgr {
    cache_entitys:HashMap<u32,NodeId>,
    scene_tree:SceneOctree
}

impl SceneOctreeMgr {
    pub fn new(aabb:AABB3) -> Self {
        let scene_tree = SceneOctree::new(aabb.min, aabb.max);
        SceneOctreeMgr { scene_tree,cache_entitys:Default::default() }
    }

    pub fn add(&mut self,entity:Entity,aabb:AABB3) {

    }
}



pub(crate) fn on_post_startup(mut commands:Commands,scene:Res<SceneEnv>) {
    let scene_oct_mgr = SceneOctreeMgr::new(scene.aabb.clone());
     commands.insert_resource(scene_oct_mgr);
}

pub(crate) fn on_after_update(mgr:ResMut<SceneOctreeMgr>,meshes:Res<Assets<Mesh>>,all_renders:Query<(Entity,&Transform,&Handle<Mesh>,&Handle<Material>,Option<&EInfo>)>) {
    for (entity,t,hmesh,_,info) in all_renders.iter() {
        //add
        if !mgr.cache_entitys.contains_key(&entity.id()) {
           if let Some(mesh) = meshes.get(&hmesh.id) {
                
           }   
        }

        //update
    }
}