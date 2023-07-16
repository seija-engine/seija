use std::collections::HashSet;
use bevy_ecs::{prelude::{Changed, Entity, Query,ParamSet}, world::World};
use seija_core::info::EStateInfo;

use crate::{hierarchy::{Children, Parent}, Transform, TransformMatrix, transform::TRANSFORM_MAT_ID};



pub fn update_transform_system(
    parent_query: Query<(Entity,Option<&Parent>)>,
    child_query:Query<Option<&Children>>,
    mut params:ParamSet<(Query<Entity,Changed<Transform>>,Query<&mut Transform>)>
) {
    if params.p1().is_empty() { return; }
    let mut changed_set:HashSet<Entity> = HashSet::new(); 
    let changed_query = params.p0();
    for e_changed in changed_query.iter() {
        let  top_entity = cacl_top_changed(e_changed, &parent_query, &changed_query);
        
        changed_set.insert(top_entity);
    }
    for top_entity in changed_set.iter() {
        let p_trans = if let Ok((_,Some(p))) = parent_query.get(*top_entity) {
            if let Ok(v) = params.p1().get(p.0) {
                v.global().clone()
            } else { TRANSFORM_MAT_ID.clone() }
        } else { TRANSFORM_MAT_ID.clone() };
       
        update_transform(*top_entity,&p_trans,&mut params,&child_query);
    }
}

fn update_transform(entity:Entity,parent:&TransformMatrix,
                    params:&mut ParamSet<(Query<Entity,Changed<Transform>>,
                    Query<&mut Transform>)>,child_query:&Query<Option<&Children>>) {
    let parent_trans = if let Ok(mut t) = params.p1().get_mut(entity) {
        t.global = parent.mul_transform(&t.local);
        t.global.clone()
    } else { TransformMatrix::default() };
   
    if let Ok(Some(childs)) = child_query.get(entity) {
        for child in childs.iter() {
           
            update_transform(*child,&parent_trans,params,&child_query);
        }
    }
}

fn cacl_top_changed(
                    e:Entity,
                    parent_query:  &Query<(Entity,Option<&Parent>)>,
                    changed_query: &Query<Entity,Changed<Transform>>) -> Entity {
    let mut top_entity = e;
    let mut cur_entity = e;
    while let Ok((_,parent)) = parent_query.get(cur_entity) {
        if let Some(p) = parent {
            if changed_query.contains(p.0) {
                top_entity = p.0
            }
            cur_entity = p.0;
        } else { break; }
    }
    top_entity
}

pub(crate) fn clear_delete_entity(world:&mut World) {
   let mut entitys = world.query::<(Entity,&EStateInfo)>();
   let mut delete_list = vec![];
   for (entity,info) in entitys.iter(world) {
    if info.is_delete {
        delete_list.push(entity);
    }
   }
   for e in delete_list {
      world.despawn(e);
   }
}