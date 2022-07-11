use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::{Changed, Entity, Query, With, Without, Commands};
use smallvec::SmallVec;
use crate::{hierarchy::{Children, Parent, PreviousParent}, Transform, TransformMatrix, transform::TRANSFORM_MAT_ID};



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

    let mut children_additions = HashMap::<Entity, SmallVec<[Entity; 8]>>::default();
    for (entity, parent, opre_parent) in parent_query.iter_mut() {
        if let Some(mut pre_parent) = opre_parent {
            if pre_parent.0 == parent.0 { continue; }
            //父节点变更后
            
            //修改PreviousParent以及把自己从旧父节点的Children中移除
            if let Ok(mut pre_parent_children) = children_query.get_mut(pre_parent.0) {
                (*pre_parent_children).0.retain(|e| *e != entity);
            }
            *pre_parent = PreviousParent(parent.0);
        } else {
            commands.entity(entity).insert(PreviousParent(parent.0));
        }
        
        //把自己插入到新的父节点的Children里
        if let Ok(mut new_parent_children) = children_query.get_mut(parent.0) {
            (*new_parent_children).0.push(entity);
        } else {
            children_additions.entry(parent.0).or_insert_with(Default::default).push(entity);
        }
    }

    children_additions.iter().for_each(|(e, v)| {
        commands.entity(*e).insert(Children::from(v));
    });
}


pub(crate) fn update_transform_system(
    changed_query: Query<Entity,Changed<Transform>>,
    parent_query: Query<(Entity,Option<&Parent>)>,
    child_query:Query<Option<&Children>>,
    mut t_query:Query<&mut Transform>,
) {
    if changed_query.is_empty() { return; }
    let mut changed_set:HashSet<Entity> = HashSet::new(); 
    for e_changed in changed_query.iter() {
        let  top_entity = cacl_top_changed(e_changed, &parent_query, &changed_query);
        changed_set.insert(top_entity);
    }

    for top_entity in changed_set.iter() {
        update_transform(*top_entity,&TRANSFORM_MAT_ID,&mut t_query,&child_query);
    }
}

fn update_transform(entity:Entity,parent:&TransformMatrix,t_query:&mut Query<&mut Transform>,child_query:&Query<Option<&Children>>) {
    let parent_trans = if let Ok(mut t) = t_query.get_mut(entity) {
        t.global = t.local.mul_transform(parent);
        t.global.clone()
    } else { TransformMatrix::default() };

    if let Ok(Some(childs)) = child_query.get(entity) {
        for child in childs.iter() {
            update_transform(*child,&parent_trans,t_query,&child_query);
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