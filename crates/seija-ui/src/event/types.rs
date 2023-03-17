use std::collections::HashMap;
use bitflags::bitflags;
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

bitflags! {
    pub struct EventNodeState: u32 {
         const NONE     = 0b00000000;
         const TOUCH_IN = 0b00000001;
     }
 }

#[derive(Component,Default)]
pub struct EventNode {
    pub state:u32,
    pub event_type:UIEventType,
    pub stop_capture:bool,
    pub stop_bubble:bool,
    ///是否使用捕获模式
    pub use_capture:bool,
    pub user_key:Option<SmolStr> 
}

impl EventNode {
    pub fn clear_mark(&mut self) {
        self.state = EventNodeState::NONE.bits();
    }

    pub fn is_touch_in(&self) -> bool {
        self.state & EventNodeState::TOUCH_IN.bits() > 0
    }
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