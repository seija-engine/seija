use bevy_ecs::{prelude::*, system::SystemParam};
use seija_core::{math::{Vec4, Vec2}, window::AppWindow};
use seija_input::{Input};
use seija_transform::{Transform, hierarchy::{Children, Parent}};
use crate::{components::rect2d::Rect2D};
use super::{UIEventSystem, EventNode, UIEventType, EventNodeState, UIEvent};

#[derive(SystemParam)]
pub struct EventParams<'w,'s> {
    pub(crate) input:Res<'w,Input>,
    pub(crate) infos:Query<'w,'s,(Entity,Option<&'static Rect2D>,&'static Transform)>,
    pub(crate) ui_systems:Query<'w,'s,(Entity,&'static UIEventSystem)>,
    pub(crate) childs:Query<'w,'s,&'static Children>,
    pub(crate) window:Res<'w,AppWindow>,
    pub(crate) parent:Query<'w,'s,&'static Parent>,
}

pub fn ui_event_system(params:EventParams,mut event_nodes:Query<(Entity,&mut EventNode)>,mut sender:EventWriter<UIEvent>) {
    for (entity,ui_system) in params.ui_systems.iter() {
      ui_system_handle(entity, ui_system, &params,&mut event_nodes,&mut sender);
    }
}


pub fn ui_system_handle(system_entity:Entity,_:&UIEventSystem,params:&EventParams,event_nodes:&mut Query<(Entity,&mut EventNode)>,sender:&mut EventWriter<UIEvent>) {
    if params.input.has_mouse_down() || params.input.has_mouse_up() {
        let mouse_pos = mouse_pos_to_world(params.input.mouse_position, system_entity, params);

        let fire_type = if params.input.has_mouse_down() {
            UIEventType::TOUCH_START
        } else {
            UIEventType::TOUCH_END
        };
        capture_ui_system(system_entity,fire_type,mouse_pos,params,event_nodes,sender);
        
        if params.input.has_mouse_up() {
            for (_,mut event_node) in event_nodes.iter_mut() {
                event_node.state &= !EventNodeState::TOUCH_IN.bits();
            }
        }
    }

    if params.input.is_mouse_move {
        let mouse_pos = mouse_pos_to_world(params.input.mouse_position, system_entity, params);
        for (entity,mut event_node) in event_nodes.iter_mut() {
            let mut is_in_rect = false;
            if let Ok((_,Some(rect),t)) = params.infos.get(entity) {
                if rect.test(t.global(), mouse_pos) {
                    is_in_rect = true;
                }
            }
            if event_node.event_type.bits() & UIEventType::MOUSE_ENTER.bits() != 0 {
                if event_node.state & EventNodeState::MOVE_IN.bits() == 0 {
                    if is_in_rect {
                        event_node.state |= EventNodeState::MOVE_IN.bits();
                        sender.send(UIEvent {
                            entity:entity,
                            event_type:UIEventType::MOUSE_ENTER,
                            user_key:event_node.user_key.clone(),
                        });  
                    }
                }
            }
            if event_node.event_type.bits() & UIEventType::MOUSE_LEAVE.bits() != 0 {
                if event_node.state & EventNodeState::MOVE_IN.bits() != 0 {
                    if !is_in_rect {
                        event_node.state &= !EventNodeState::MOVE_IN.bits();
                        sender.send(UIEvent {
                            entity,
                            event_type:UIEventType::MOUSE_LEAVE,
                            user_key:event_node.user_key.clone(),
                        });  
                    }
                }
            }
            
        }
    }
}

fn mouse_pos_to_world(mut mouse_pos:Vec2,entity:Entity,params:&EventParams) -> Vec2 {
    mouse_pos.x -= params.window.width() as f32 * 0.5f32;
    mouse_pos.y = params.window.height() as f32 * 0.5f32 - mouse_pos.y;
    if let Ok((_,_,transform)) = params.infos.get(entity) {
        let new_pos = transform.global().matrix() * Vec4::new(mouse_pos.x,mouse_pos.y,0.0,1.0);
        return Vec2::new(new_pos.x, new_pos.y);
    }
    mouse_pos
}

fn capture_ui_system(system_entity:Entity,fire_type:UIEventType,
                     mouse_pos:Vec2,params:&EventParams,
                     event_nodes:&mut Query<(Entity,&mut EventNode)>,
                     sender:&mut EventWriter<UIEvent>) {
    if let Ok(system_child) = params.childs.get(system_entity) {
        for child_entity in system_child.iter() {
           let target_entity = capture_event_node(*child_entity,fire_type,mouse_pos,false,params, event_nodes,sender);
           if let Some(target_entity) = target_entity {
               bubble_event_node(target_entity, fire_type, false, mouse_pos, params, event_nodes, sender);
           }
        }
    }
}

fn capture_event_node(event_entity:Entity,
                      fire_type:UIEventType,
                      mouse_pos:Vec2,
                      mut stop_capture:bool,
                      params:&EventParams,
                      event_nodes:&mut Query<(Entity,&mut EventNode)>,
                      sender:&mut EventWriter<UIEvent>) -> Option<Entity> {
    let mut last_hit_entity = None;
    if let Err(err) =  params.infos.get(event_entity) {
        log::error!("capture_event_node error:{:?}",err);
    }
    if let Ok((_,Some(rect2d),t)) = params.infos.get(event_entity) {
        if !rect2d.test(t.global(), mouse_pos) {
            return None;
        }
        last_hit_entity = Some(event_entity);
        
        if let Ok((_,mut event_node)) = event_nodes.get_mut(event_entity) {
            if !stop_capture {
                if event_node.stop_capture {
                    stop_capture = true;
                }
                let is_click = (event_node.event_type.bits() & UIEventType::CLICK.bits() != 0) && 
                               fire_type == UIEventType::TOUCH_END 
                               && (event_node.state & EventNodeState::TOUCH_IN.bits() != 0);
                let is_start_or_end = event_node.event_type.bits() & fire_type.bits() != 0;
                if event_node.use_capture {
                    if is_start_or_end {
                        sender.send(UIEvent {
                            entity:event_entity,
                            event_type:fire_type,
                            user_key:event_node.user_key.clone(),
                        });
                    }
                    if is_click {
                        sender.send(UIEvent {
                            entity:event_entity,
                            event_type:UIEventType::CLICK,
                            user_key:event_node.user_key.clone(),
                        });
                    }
                }
            }

            if fire_type == UIEventType::TOUCH_START {
                event_node.state |= EventNodeState::TOUCH_IN.bits();
            }
        }
        
        if let Ok(child) = params.childs.get(event_entity) {
            for child_entity in child.iter() {
                let capture_entity = capture_event_node(*child_entity,fire_type,mouse_pos,stop_capture, params, event_nodes,sender);
                if capture_entity.is_some() {
                    last_hit_entity = capture_entity;
                }
            }
        }
    } else {
        
    }
    last_hit_entity
}


fn bubble_event_node(event_entity:Entity,
                     fire_type:UIEventType,
                     mut stop_bubble:bool,
                     mouse_pos:Vec2,
                     params:&EventParams,
                     event_nodes:&mut Query<(Entity,&mut EventNode)>,
                     sender:&mut EventWriter<UIEvent>) {
    
    if let Ok((_,event_node)) = event_nodes.get_mut(event_entity) {
        if !stop_bubble {
            let is_click =  (event_node.event_type.bits() & UIEventType::CLICK.bits() != 0) && 
                            fire_type == UIEventType::TOUCH_END && 
                            (event_node.state & EventNodeState::TOUCH_IN.bits() != 0);
            let is_start_or_end = event_node.event_type.bits() & fire_type.bits() != 0;
            if event_node.use_capture == false {
                if is_start_or_end {
                    sender.send(UIEvent {
                        entity:event_entity,
                        event_type:fire_type,
                        user_key:event_node.user_key.clone(),
                    });
                }
                if is_click {
                    sender.send(UIEvent {
                        entity:event_entity,
                        event_type:UIEventType::CLICK,
                        user_key:event_node.user_key.clone(),
                    });
                }
            }
            if event_node.stop_bubble {
                stop_bubble = true;
            }  
        }
    }

    if let Ok(parent) = params.parent.get(event_entity) {
        bubble_event_node(parent.0,fire_type,stop_bubble,mouse_pos,params,event_nodes,sender);
    }
}