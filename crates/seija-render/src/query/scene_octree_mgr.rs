use std::collections::HashMap;

use bevy_ecs::{system::{Res, Commands, Query, ResMut}, prelude::Entity};
use glam::Vec3;
use seija_asset::{Handle, Assets};
use seija_core::info::EInfo;
use seija_geometry::{volume::AABB3, Contains};
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

    pub fn add(&mut self,entity:Entity,aabb:Option<AABB3>) {
       let  add_aabb = aabb.unwrap_or(self.scene_tree.nodes[0].aabb.clone());
       let id = self.scene_tree.add(entity, add_aabb);
       self.cache_entitys.insert(entity.id(), id);
    }
}



pub(crate) fn on_post_startup(mut commands:Commands,scene:Res<SceneEnv>) {
    let scene_oct_mgr = SceneOctreeMgr::new(scene.aabb.clone());
     commands.insert_resource(scene_oct_mgr);
}

pub(crate) fn on_after_update(mut mgr:ResMut<SceneOctreeMgr>,
                              meshes:Res<Assets<Mesh>>,
                              all_renders:Query<(Entity,&Transform,&Handle<Mesh>,&Handle<Material>,Option<&EInfo>)>) {
    for (entity,t,hmesh,_,info) in all_renders.iter() {
        //add
        if !mgr.cache_entitys.contains_key(&entity.id()) {
           if let Some(mesh) = meshes.get(&hmesh.id) {
            if let Some(aabb) = mesh.aabb.as_ref() {
                let real_aabb = aabb.transform(&t.global().matrix());
                log::error!("add:{:?}",&t.global());
                mgr.add(entity, Some(real_aabb));
            } else {
                mgr.add(entity, None);
            }
           } else {
                log::error!("mesh id invalid {:?}",hmesh);
           }
        }

        //update
    }
}
