use std::collections::{HashSet};

use bevy_ecs::prelude::{Changed, Entity, Query,Without, Commands, ParamSet};

use crate::{hierarchy::{Children, Parent, PreviousParent}, Transform, TransformMatrix, transform::TRANSFORM_MAT_ID};

/*

pub(crate) fn parent_update_system(
                                   mut commands: Commands,
                                   mut children_query: Query<&mut Children>,
                                   mut parent_query: Query<(Entity, &Parent, Option<&mut PreviousParent>)>,
                                   removed_parent_query: Query<(Entity, &PreviousParent), Without<Parent>>
                                  ) {
    //parent被移除后同步移除PreviousParent以及父节点的Children
    for (entity, previous_parent) in removed_parent_query.iter() {
        if let Ok(mut previous_parent_children) = children_query.get_mut(previous_parent.0) {
            previous_parent_children.0.retain(|e| *e != entity);
            commands.entity(entity).remove::<PreviousParent>();
        }
    }

    for (entity, parent, pre_parent) in parent_query.iter_mut() {
        if let Some(mut pre_parent) = pre_parent {
            //上一个和当前相同跳过
            if pre_parent.0 == parent.0 {
                continue; 
            }
            //父节点变更后
            *pre_parent = PreviousParent(parent.0);
        } else {
            //首次创建同步和Parent创建一个PreviousParent
            commands.entity(entity).insert(PreviousParent(parent.0));
        }
        
        
    }
   
}
*/

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

fn update_transform(entity:Entity,parent:&TransformMatrix,params:&mut ParamSet<(Query<Entity,Changed<Transform>>,Query<&mut Transform>)>,child_query:&Query<Option<&Children>>) {
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