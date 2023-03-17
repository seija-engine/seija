use bevy_ecs::{prelude::*, system::SystemParam};
use seija_core::{math::{Vec3, Vec4, Vec2}, window::AppWindow};
use seija_input::{Input, event::MouseButton};
use seija_transform::{Transform, hierarchy::{Children, Parent}};
use crate::{components::rect2d::Rect2D};
use super::{UIEventSystem, EventNode, UIEventType, EventNodeState};

#[derive(SystemParam)]
pub struct EventParams<'w,'s> {
    pub(crate) input:Res<'w,Input>,
    pub(crate) infos:Query<'w,'s,(Entity,Option<&'static Rect2D>,&'static Transform)>,
    pub(crate) ui_systems:Query<'w,'s,(Entity,&'static UIEventSystem)>,
    pub(crate) childs:Query<'w,'s,&'static Children>,
    pub(crate) window:Res<'w,AppWindow>,
    pub(crate) parent:Query<'w,'s,&'static Parent>
}

pub fn ui_event_system(params:EventParams,mut event_nodes:Query<(Entity,&mut EventNode)>) {
    event_nodes.for_each_mut(|(_,mut event_node)| { event_node.clear_mark(); });
    for (entity,ui_system) in params.ui_systems.iter() {
      ui_system_handle(entity, ui_system, &params,&mut event_nodes);
    }
}


pub fn ui_system_handle(system_entity:Entity,_:&UIEventSystem,params:&EventParams,event_nodes:&mut Query<(Entity,&mut EventNode)>) {
    if params.input.has_mouse_down() || params.input.has_mouse_up() {
        let mut mouse_pos = params.input.mouse_position;
        mouse_pos.x -= params.window.width() as f32 * 0.5f32;
        mouse_pos.y = params.window.height() as f32 * 0.5f32 - mouse_pos.y;
        if let Ok((_,_,transform)) = params.infos.get(system_entity) {
            let new_pos = transform.global().matrix() * Vec4::new(mouse_pos.x,mouse_pos.y,0.0,1.0);
            mouse_pos.x = new_pos.x;
            mouse_pos.y = new_pos.y;
        }
        mark_ui_system(system_entity,mouse_pos,params,event_nodes);
    }
    
    for (event_entity,event_node) in event_nodes.iter() {
        match event_node.event_type {
            UIEventType::TouchStart => { 
                if params.input.has_mouse_down() && event_node.state & EventNodeState::TOUCH_IN.bits() > 0 { 
                    try_fire_event_node(event_entity,event_node,event_nodes,params); 
                }
            },
            UIEventType::TouchEnd => {

            },
            UIEventType::Click => {
                
            },
            UIEventType::MouseEnter => {

            },
            UIEventType::MouseLeave => {

            }

        }  
    }
}


fn mark_ui_system(system_entity:Entity,mouse_pos:Vec2,params:&EventParams,event_nodes:&mut Query<(Entity,&mut EventNode)>) {
    if let Ok(system_child) = params.childs.get(system_entity) {
        system_child.iter().for_each(|entity| { mark_event_node(*entity, mouse_pos, params, event_nodes) });
    }
}

fn mark_event_node(event_entity:Entity,mouse_pos:Vec2,params:&EventParams,event_nodes:&mut Query<(Entity,&mut EventNode)>) {
    if let Ok((_,Some(rect2d),t)) = params.infos.get(event_entity) {
        if !rect2d.test(t.global(), mouse_pos) {
            return;
        }
        if let Ok(mut event_node) = event_nodes.get_mut(event_entity) {
            event_node.1.state = EventNodeState::TOUCH_IN.bits();
        }
        if let Ok(child_entity) = params.childs.get(event_entity) {
            child_entity.iter().for_each(|entity| { mark_event_node(*entity, mouse_pos, params, event_nodes) });
        }
    }

}


fn try_fire_event_node(entity:Entity,event_node:&EventNode,event_nodes:&Query<(Entity,&mut EventNode)>,params:&EventParams) {
    if event_node.use_capture {
        let mut cur_entity = entity;
        while let Ok(parent) = params.parent.get(cur_entity) {
            if let Ok(cur_event_node) = event_nodes.get(cur_entity) {
                if cur_event_node.1.stop_capture && cur_event_node.1.is_touch_in() {
                    return;
                }
            }
            cur_entity = parent.0;
        }
        seija_core::log::error!("send event:{:?}-{:?}",entity,event_node.user_key);
    } else {
       if has_child_stop_bubble(true, entity, event_nodes, params) { return; }
       seija_core::log::error!("send event:{:?}-{:?}",entity,event_node.user_key);
    }
}

fn has_child_stop_bubble(is_self:bool,entity:Entity,event_nodes:&Query<(Entity,&mut EventNode)>,params:&EventParams) -> bool {
    if !is_self {   
        if let Ok(event_node) = event_nodes.get(entity) {
            if event_node.1.is_touch_in() && event_node.1.stop_bubble {
                return true;
            }
        }
    }
    if let Ok(child) = params.childs.get(entity) {
        for child_entity in child.iter() {
           let has_stop_bubble = has_child_stop_bubble(false,*child_entity, event_nodes, params);
           if has_stop_bubble {
                return true;
           }
        }
    }
    false
}