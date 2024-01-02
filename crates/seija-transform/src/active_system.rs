use bevy_ecs::{system::{Query, ParamSet, Commands}, prelude::{Entity, EventReader}, query::Added};
use seija_core::info::EStateInfo;
use crate::{hierarchy::{Children, Parent}, events::HierarchyEvent};

pub(crate) fn active_add_system(parent_query:Query<(Entity,Option<&Parent>)>,
                                child_query:Query<Option<&Children>>,mut commands:Commands,
                                mut params:ParamSet<(Query<Entity,Added<EStateInfo>>,Query<&mut EStateInfo>)>) {
    if params.p0().is_empty() { return; }
   
    let mut dirty_entitys:Vec<Entity> = params.p0().iter().collect();
    for entity in dirty_entitys.drain(..) {
        let mut p_active = true;
        if let Some(p) = parent_query.get(entity).ok().and_then(|v| v.1) {
            if let Ok(info) = params.p1().get(p.0) {
                p_active = info._is_active_global
            }
        }
        let cur_active = if p_active == false { p_active } else {
            params.p1().get(entity).map(|v| v.is_active_global()).unwrap_or(true)
        };
        _set_children_active(entity, cur_active,&mut params.p1(),&child_query,&mut commands)
    }
}

pub(crate) fn active_system(parent_query:Query<(Entity,Option<&Parent>)>,
                            child_query:Query<Option<&Children>>,
                            mut state_infos:Query<&mut EStateInfo>,
                            mut active_reader:EventReader<HierarchyEvent>,mut commands:Commands) {
    for event in active_reader.iter() {
        if let HierarchyEvent::SetActive(entity, new_active) = event {
            //log::error!("active_system set {:?}={:?}",entity,new_active);
            let mut new_active:bool = *new_active;
            let mut p_active = true;
            if let Some(p) = parent_query.get(*entity).ok().and_then(|v| v.1) {
                if let Ok(info) = state_infos.get(p.0) {
                    p_active = info.is_active_global()
                }
            }                
            if p_active == false { new_active = false }
            _set_children_active(*entity, new_active,&mut state_infos,&child_query,&mut commands)
        }
    }
}

fn _set_children_active(entity:Entity,active:bool,state_infos:&mut Query<&mut EStateInfo>,
                        childs:&Query<Option<&Children>>,commands:&mut Commands) {
    //log::error!("real set active:{:?} = {:?}",entity,active);
    let mut new_active = active;
    if let Ok(mut sinfo) = state_infos.get_mut(entity) {
        if sinfo.is_active() {
            sinfo._is_active_global = active;
        } else {
            new_active = false;
        }
    } else {
        let mut info = EStateInfo::default();
        info._is_active_global = active;
        commands.entity(entity).insert(info);
    }
    if let Ok(Some(child)) = childs.get(entity) {
        for child_entity in child.iter() {
            _set_children_active(*child_entity, new_active, state_infos, childs,commands)
        }
    }
}