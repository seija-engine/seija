use bevy_ecs::{system::Query, prelude::{Entity, EventReader}};
use seija_core::info::EStateInfo;
use crate::{hierarchy::{Children, Parent}, events::HierarchyEvent};


pub(crate) fn active_system(parent_query:Query<(Entity,Option<&Parent>)>,
                            child_query:Query<Option<&Children>>,
                            mut state_infos:Query<(Entity,&mut EStateInfo)>,
                            mut active_reader:EventReader<HierarchyEvent>) {
    for event in active_reader.iter() {
        if let HierarchyEvent::SetActive(entity, new_active) = event {
            let mut new_active:bool = *new_active;
            let mut p_active = true;
            if let Some(p) = parent_query.get(*entity).ok().and_then(|v| v.1) {
                if let Ok(info) = state_infos.get(p.0) {
                    p_active = info.1._is_active_global
                }
            }                
            if p_active == false { new_active = false }
            _set_children_active(*entity, new_active,&mut state_infos,&child_query)
        }
    }
}

fn _set_children_active(entity:Entity,active:bool,state_infos:&mut Query<(Entity,&mut EStateInfo)>,childs:&Query<Option<&Children>>) {
    if let Ok(mut state_info) = state_infos.get_mut(entity) {
        state_info.1._is_active_global = active;
    }
    if let Ok(Some(child)) = childs.get(entity) {
        for child_entity in child.iter() {
            _set_children_active(*child_entity, active, state_infos, childs)
        }
    }
}