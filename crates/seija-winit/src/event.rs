use winit::event::{KeyboardInput, ElementState, MouseButton};
use seija_input::{event::{KeyboardInput as IKeyboardInput,InputState, MouseInput,MouseButton as IMouseButton}, keycode::KeyCode};

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
            
            ElementState::Pressed =>InputState::Pressed,
            ElementState::Released =>InputState::Released,
        }
     }
}

fn conv_input_state(state:ElementState) -> InputState {
    match state {
        ElementState::Pressed => InputState::Pressed,
        ElementState::Released => InputState::Released
    }
}

pub(crate) fn conv_mouse_input(state:ElementState,button:MouseButton) -> MouseInput {
    MouseInput {
        state:conv_input_state(state),
        button:match button {
            MouseButton::Left => IMouseButton::Left,
            MouseButton::Right => IMouseButton::Right,
            MouseButton::Middle => IMouseButton::Middle,
            MouseButton::Other(v) => IMouseButton::Other(v),
        }
    }
}