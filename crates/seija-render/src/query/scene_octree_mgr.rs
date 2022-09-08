use std::collections::HashMap;

use bevy_ecs::{system::{Res, Commands, Query, ResMut,  RemovedComponents}, prelude::Entity, query::Changed};

use seija_asset::{Handle, Assets};
use seija_core::info::EInfo;
use seija_geometry::{volume::AABB3};
use seija_transform::Transform;

use crate::{scene::SceneEnv, material::Material, resource::Mesh};

use super::scene_octree::{SceneOctree, NodeId};

pub struct SceneOctreeMgr {
    cache_entitys:HashMap<u32,NodeId>,
    pub scene_tree:SceneOctree
}

impl SceneOctreeMgr {
    pub fn new(aabb:AABB3) -> Self {
        let scene_tree = SceneOctree::new(aabb.min, aabb.max);
        SceneOctreeMgr { scene_tree,cache_entitys:Default::default() }
    }

    pub fn add(&mut self,entity:Entity,aabb:Option<AABB3>) -> NodeId {
       let  add_aabb = aabb.clone().unwrap_or(self.scene_tree.nodes[0].aabb.clone());
       let id = self.scene_tree.add(entity, add_aabb);
       self.cache_entitys.insert(entity.id(), id);
       log::info!("octree add:{} {:?}",id,aabb);
       id
    }

    pub fn has(&self,eid:u32) -> bool {
        self.cache_entitys.contains_key(&eid)
    }

    pub fn update(&mut self,entity:Entity,new_aabb:Option<AABB3>) -> Option<NodeId> {
        let update_aabb = new_aabb.unwrap_or(self.scene_tree.nodes[0].aabb.clone());
        
        if let Some(node_id) = self.cache_entitys.get(&entity.id()) {
          if let Some(update_id) = self.scene_tree.update(*node_id, entity, update_aabb) {
             self.cache_entitys.insert(entity.id(), update_id);
             return Some(update_id);
          }
          Some(*node_id)
        } else {
            None
        }
    }


    pub fn remove(&mut self,entity:Entity) {
        if let Some(node_id) = self.cache_entitys.remove(&entity.id()) {
            self.scene_tree.remove_bynode(node_id, entity);
        }
    }
}



pub(crate) fn on_post_startup(mut commands:Commands,scene:Res<SceneEnv>) {
    let scene_oct_mgr = SceneOctreeMgr::new(scene.aabb.clone());
     commands.insert_resource(scene_oct_mgr);
}

pub(crate) fn on_last_update(mut mgr:ResMut<SceneOctreeMgr>,
                              meshes:Res<Assets<Mesh>>,
                              removed:RemovedComponents<Handle<Mesh>>,
                              changed:Query<Entity,Changed<Transform>>,
                              all_renders:Query<(Entity,&Transform,&Handle<Mesh>,&Handle<Material>,Option<&EInfo>)>) {
    for (entity,t,hmesh,_,_) in all_renders.iter() {
        //add
        if !mgr.cache_entitys.contains_key(&entity.id()) {
           if let Some(mesh) = meshes.get(&hmesh.id) {
            if let Some(aabb) = mesh.aabb.as_ref() {
                let real_aabb = aabb.transform(&t.global().matrix());
                mgr.add(entity, Some(real_aabb));
            } else {
                mgr.add(entity, None);
            }

             //update
            if changed.contains(entity) {
                let new_aabb = mesh.aabb.as_ref().map(|v| v.transform(&t.global().matrix()));
                mgr.update(entity, new_aabb);
            }
           } else {
                log::error!("mesh id invalid {:?}",hmesh);
           }
        }  
    }
    //remove
    for rm in removed.iter() {
        if mgr.cache_entitys.contains_key(&rm.id()) {
            mgr.remove(rm);
        }
    }
}
