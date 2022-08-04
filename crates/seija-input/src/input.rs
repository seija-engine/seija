use std::collections::HashSet;

use crate::event::KeyCode;
#[derive(Default)]
pub struct Input {
    pub(crate) key_pressing:HashSet<KeyCode>,
    pub(crate) frame_keydown:HashSet<KeyCode>,
    pub(crate) frame_keyup:HashSet<KeyCode>
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


    pub(crate) fn clear(&mut self) {
        self.frame_keydown.clear();
        self.frame_keyup.clear();
    }
}