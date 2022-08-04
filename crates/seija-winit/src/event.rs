use winit::event::{KeyboardInput, ElementState};
use seija_input::event::{KeyboardInput as IKeyboardInput,KeyboardInputState, KeyCode};

#[derive(Debug, Clone,Copy)]
pub struct WindowResized {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone,Copy)]
pub struct WindowCreated;

pub(crate) fn conv_keyboard_input(key:KeyboardInput) -> IKeyboardInput {
    let key_code = match key.virtual_keycode {
        Some(v) => {
            let u32_key = v as u32;
            unsafe {KeyCode::from_u32(u32_key) }
        },
        None => KeyCode::Unknow,
    };
    IKeyboardInput {
        key_code,
        scan_code:key.scancode,
        state:match key.state {
            
            ElementState::Pressed =>KeyboardInputState::Pressed,
            ElementState::Released =>KeyboardInputState::Released,
        }
     }
}