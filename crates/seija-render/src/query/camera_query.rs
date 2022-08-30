use bevy_ecs::{prelude::{Entity, Query, ResMut, Res, Added, RemovedComponents}};
use seija_asset::{Handle, Assets};
use seija_core::info::EInfo;
use seija_transform::Transform;


use crate::{material::{Material}, camera::camera::Camera};

use super::{view_list::{ViewEntity}, QuerySystem, ViewQuery, system::IdOrName};

const CAMERA_TYPE:u32 = 1u32;



pub(crate) fn camera_query_check_add(mut system:ResMut<QuerySystem>,
                                     add_cameras:Query<Entity,Added<Camera>>,
                                     remove_cameras:RemovedComponents<Camera>) {
    for add_camera in add_cameras.iter() {
        let eid = add_camera.to_bits();
        
        system.add_query(IdOrName::Id(eid), CAMERA_TYPE);
    }
    for rm in remove_cameras.iter() {
        system.rmove_query(IdOrName::Id(rm.to_bits()));
    }
}

pub(crate) fn camera_query_update(system:Res<QuerySystem>,
                                  query:Query<(Entity,&Transform,&Handle<Material>,Option<&EInfo>)>,
                                  mats:Res<Assets<Material>>,
                                  cameras:Query<(&Camera,&Transform)>) {
   for view_query in system.querys.iter() {
        if view_query.read().typ == CAMERA_TYPE {
            update_camera_query(&mut view_query.write(),&query,&mats,&cameras);
        }
   }
}

fn update_camera_query(view_query:&mut ViewQuery,
                       query:&Query<(Entity,&Transform,&Handle<Material>,Option<&EInfo>)>,
                       materials:&Assets<Material>,
                       cameras:&Query<(&Camera,&Transform)>) -> Option<()> {
    view_query.list.clear();
    let id = Entity::from_bits(view_query.key.cast_id()?);
    let (camera,t) = cameras.get(id).ok()?;
    let camera_position = t.global().position;
    
    for (entity,t,m,info) in query.iter() {
        if let Some(info) = info {
            if info.layer & camera.layer  < 1 { continue; }
        }
        let position = t.global().position;
        let dis_order = (camera_position - position).length_squared();
        let mat = materials.get(&m.id)?;
        view_query.list.add_entity(mat.order, ViewEntity { entity, order:dis_order });
    }
    view_query.list.sort();
    Some(())
}