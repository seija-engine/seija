use glam::Vec2;

use crate::keycode::KeyCode;



#[derive(Debug, Clone)]
pub struct KeyboardInput {
    pub scan_code: u32,
    pub key_code:KeyCode,
    pub state:InputState
}

#[derive(Debug, Clone)]
pub enum InputState {
    Pressed,
    Released,
    
}
#[derive(Debug, Clone)]
pub struct MouseInput {
    pub button:MouseButton,
    pub state:InputState
}

#[derive(Debug, Clone, Default)]
pub struct MouseWheelInput {
    pub delta:Vec2
}


#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u32),
}

impl From<u32> for MouseButton {
    fn from(num: u32) -> Self {
        match num {
           0 => MouseButton::Left,
           1 => MouseButton::Right,
           2 => MouseButton::Middle,
           n => MouseButton::Other(n)
        }
    }
}