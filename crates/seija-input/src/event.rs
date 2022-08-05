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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}
