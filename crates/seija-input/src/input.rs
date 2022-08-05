use std::collections::HashSet;
use glam::Vec2;

use crate::{keycode::KeyCode, event::MouseButton};

#[derive(Default)]
pub struct Input {
    pub mouse_position:Vec2,
    pub mouse_wheel:Vec2,
    pub(crate) key_pressing:HashSet<KeyCode>,
    pub(crate) frame_keydown:HashSet<KeyCode>,
    pub(crate) frame_keyup:HashSet<KeyCode>,

    pub(crate) frame_mousedown:HashSet<MouseButton>,
    pub(crate) frame_mouseup:HashSet<MouseButton>,
}

impl Input {
    pub fn get_key_down(&self,code:KeyCode) -> bool {
        self.frame_keydown.contains(&code)
    }

    pub fn get_key_up(&self,code:KeyCode) -> bool {
        self.frame_keyup.contains(&code)
    }

    pub fn get_key(&self,code:KeyCode) -> bool {
        self.key_pressing.contains(&code)
    }

    pub fn get_mouse_down(&self,mouse:MouseButton) -> bool {
        self.frame_mousedown.contains(&mouse)
    }

    pub fn get_mouse_up(&self,mouse:MouseButton) -> bool {
        self.frame_mouseup.contains(&mouse)
    }


    pub(crate) fn clear(&mut self) {
        self.frame_keydown.clear();
        self.frame_keyup.clear();

        self.frame_mousedown.clear();
        self.frame_mouseup.clear();
    }
}