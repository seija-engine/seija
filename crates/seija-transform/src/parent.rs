use std::collections::HashMap;

use bevy_ecs::prelude::{Commands, Entity, Query, Without};
use smallvec::SmallVec;

use crate::hierarchy::{Children, Parent, PreviousParent};



pub(crate) fn parent_update_system(mut commands: Commands,
                                   mut children_query: Query<&mut Children>,
                                   mut parent_query: Query<(Entity, &Parent, Option<&mut PreviousParent>)>,
                                   removed_parent_query: Query<(Entity, &PreviousParent), Without<Parent>>) {
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