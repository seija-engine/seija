use bevy_ecs::{prelude::*, system::SystemParam};
use seija_core::{math::{Vec3, Vec4}, window::AppWindow};
use seija_input::{Input, event::MouseButton};
use seija_transform::Transform;
use crate::components::rect2d::Rect2D;
use super::{UIEventSystem, EventNode, UIEventType};

#[derive(SystemParam)]
pub struct EventParams<'w,'s> {
    pub(crate) input:Res<'w,Input>,
    pub(crate) infos:Query<'w,'s,(Entity,Option<&'static Rect2D>,&'static Transform)>,
    pub(crate) ui_systems:Query<'w,'s,(Entity,&'static UIEventSystem)>,
    pub(crate) event_nodes:Query<'w,'s,(Entity,&'static mut EventNode)>,
    pub(crate) window:Res<'w,AppWindow>
}

pub fn ui_event_system(params:EventParams) {
    for (entity,ui_system) in params.ui_systems.iter() {
       ui_system_handle(entity, ui_system, &params);
    }
}
/*
实现思路:
1. 遍历所有的UI系统
2. 遍历所有的UI系统的事件节点
*/

pub fn ui_system_handle(system_entity:Entity,system:&UIEventSystem,params:&EventParams) {
    let mut mouse_pos = params.input.mouse_position;
    mouse_pos.x -= params.window.width() as f32 * 0.5f32;
    mouse_pos.y = params.window.height() as f32 * 0.5f32 - mouse_pos.y;
    if let Ok((_,_,transform)) = params.infos.get(system_entity) {
        let new_pos = transform.global().matrix() * Vec4::new(mouse_pos.x,mouse_pos.y,0.0,1.0);
        mouse_pos.x = new_pos.x;
        mouse_pos.y = new_pos.y;
    }
    for (event_entity,event_node) in params.event_nodes.iter() {
        if let Ok((_,Some(rect2d),transform)) = params.infos.get(event_entity) {
            match event_node.event_type {
                UIEventType::TouchStart => {
                    if params.input.get_mouse_down(MouseButton::Left) {
                        if !rect2d.test(transform.global(),mouse_pos) {
                            continue;
                        }
                        start_event_propagation(event_entity, params);
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
}

pub fn start_event_propagation(event_entity:Entity,params:&EventParams) {

}