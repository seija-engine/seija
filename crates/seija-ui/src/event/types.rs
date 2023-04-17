use bitflags::bitflags;
use bevy_ecs::prelude::{Component, Entity};
use seija_core::smol_str::SmolStr;

#[derive(Clone,Debug)]
pub struct UIEvent {
    pub entity:Entity,
    pub event_type:UIEventType,
    pub user_key:Option<SmolStr>,
}

bitflags! {
    pub struct UIEventType:u32 {
        const NONE        = 0b00000000;
        const TOUCH_START = 0b00000001;
        const TOUCH_END   = 0b00000010;
        const MOUSE_ENTER = 0b00000100;
        const MOUSE_LEAVE = 0b00001000;
        const CLICK       = 0b00010000;
    }
}

bitflags! {
    pub struct EventNodeState: u32 {
         const NONE     = 0b00000000;
         const TOUCH_IN = 0b00000001;
         const MOVE_IN  = 0b00000010;
     }
 }

#[derive(Component,Debug,Clone)]
#[repr(C)]
pub struct EventNode {
    pub state:u32,
    pub event_type:UIEventType,
    pub stop_capture:bool,
    pub stop_bubble:bool,
    ///是否使用捕获模式
    pub use_capture:bool,
    pub user_key:Option<SmolStr> 
}

impl Default for EventNode {
    fn default() -> Self {
        Self {
            state : EventNodeState::NONE.bits(),
            stop_capture:false,
            stop_bubble:false,
            use_capture:true,
            user_key:None,
            event_type:UIEventType::NONE,
        }
    }
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
}

impl Default for UIEventSystem {
    fn default() -> Self {
        Self {
        }
    }
}