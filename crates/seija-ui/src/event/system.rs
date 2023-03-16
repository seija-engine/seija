use bevy_ecs::{prelude::*, system::SystemParam};
use seija_input::{Input, event::MouseButton};
use seija_transform::Transform;
use crate::components::rect2d::Rect2D;
use super::UISystem;

#[derive(SystemParam)]
pub struct EventParams<'w,'s> {
    pub(crate) input:Res<'w,Input>,
    pub(crate) infos:Query<'w,'s,(Entity,&'static Rect2D,&'static Transform)>,
    pub(crate) ui_systems:Query<'w,'s,(Entity,&'static UISystem)>,
}

pub fn ui_event_system(params:EventParams) {
    for (entity,ui_system) in params.ui_systems.iter() {
       ui_system_handle(entity, ui_system, &params);
    }
}

pub fn ui_system_handle(entity:Entity,system:&UISystem,params:&EventParams) {
    
}