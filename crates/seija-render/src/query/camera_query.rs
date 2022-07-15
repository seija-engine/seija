use std::collections::{HashMap, HashSet};

use bevy_ecs::{prelude::{World, Entity, Query, ResMut, Res, Added, RemovedComponents}};
use parking_lot::RwLock;


use crate::{material::{MaterialStorage, Material}, camera::camera::Camera};

use super::{view_list::{ViewList, ViewEntity}, QuerySystem, ViewQuery};

const CAMERA_TYPE:u32 = 1u32;

#[derive(Default)]
pub struct CameraQueryMap {
    pub map:HashMap<u64,usize>
}

pub(crate) fn camera_query_check_add(mut camera_query_map:ResMut<CameraQueryMap>,
                                     mut system:ResMut<QuerySystem>,
                                     add_cameras:Query<Entity,Added<Camera>>,
                                     remove_cameras:RemovedComponents<Camera>) {
    let  has_remove = remove_cameras.iter().next().is_some();
    if add_cameras.is_empty() && !has_remove { return ; }
    for entity in add_cameras.iter() {
        let eid = entity.to_bits();
        if !camera_query_map.map.contains_key(&eid) {
            system.querys.push(RwLock::new(ViewQuery::new(CAMERA_TYPE, eid)));
            camera_query_map.map.insert(eid, system.querys.len());
        }
    }
    if has_remove {
        let mut rm_set:HashSet<u64> = HashSet::default();
        for rm in remove_cameras.iter() {
            let rm_eid = rm.to_bits();
            rm_set.insert(rm_eid);
            camera_query_map.map.remove(&rm_eid);
        }

        if rm_set.len() > 0 {
            let mut i = 0;
            while i < system.querys.len() {
                if rm_set.contains(&system.querys[i].read().key) {
                  system.querys.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        for (index,view_query) in system.querys.iter().enumerate() {
            let query_ref = view_query.read();
            camera_query_map.map.insert(query_ref.key, index);
        }
    }
}

pub(crate) fn camera_query_update(camera_map:Res<CameraQueryMap>,system:Res<QuerySystem>) {
    for (eid,index) in camera_map.map.iter() {
       let view_query = &system.querys[*index];
       if view_query.read().typ == CAMERA_TYPE {
        
       }
    }
}


/* 
impl CameraQuery {
    fn on_query(&self,world:&mut World,list:&mut ViewList) {
        list.clear();
        
        let mut views = world.query::<(Entity,&Transform,&Handle<Material>,Option<&EInfo>)>();
        if let Some(materials) = world.get_resource::<MaterialStorage>() {
            let mats = materials.mateials.read();
            let camera = world.entity(self.camera_entity).get::<Camera>();
            let camera_t = world.entity(self.camera_entity).get::<Transform>();
            if let (Some(camera),Some(camera_t)) = (camera,camera_t) {
                let camera_position = camera_t.global().position;
                for (entity,t,m,info) in views.iter(world) {
                    if let Some(info) = info {
                        if info.layer & camera.layer  < 1 { continue; }
                    }
                    let position = t.global().position;
                    let dis_order = (camera_position - position).length_squared();
                    let mat = mats.get(&m.id).unwrap();
                    
                    list.add_entity(mat.order, ViewEntity {entity,order:dis_order });
                }
            }
        }
        list.sort();   
    }
}*/