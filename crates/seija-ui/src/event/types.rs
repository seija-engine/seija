use std::collections::HashMap;

use bevy_ecs::prelude::{Component, Entity};
use seija_core::smol_str::SmolStr;

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum UIEventType {
    TouchStart,
    TouchEnd,
    MouseEnter,
    MouseLeave,
    Click
}

impl Default for UIEventType {
    fn default() -> Self {
        Self::TouchStart
    }
}

#[derive(Component,Default)]
pub struct EventNode {
    pub event_type:UIEventType,
    pub stop_capture:bool,
    pub stop_bubble:bool,
    pub use_capture:bool,
    pub user_key:Option<SmolStr> 
}

#[derive(Component)]
pub struct UIEventSystem {
    pub(crate) frame_rect_test:HashMap<Entity,bool>
}

impl Default for UIEventSystem {
    fn default() -> Self {
        Self {
            frame_rect_test:HashMap::new()
        }
    }
}