use glam::Vec2;
use seija_app::{App, ecs::world::World};
use crate::{InputModule, Input, keycode::KeyCode};

#[no_mangle]
pub unsafe extern "C" fn input_add_module(app_ptr:*mut App) {
    (&mut *app_ptr).add_module(InputModule);
}


#[no_mangle]
pub unsafe extern "C" fn input_world_get_input(world:*const World) -> *const Input {
    if let Some(input) = (&*world).get_resource::<Input>() {
        input as *const Input
    } else {
        std::ptr::null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn input_get_keydown(input:*const Input,keycode:KeyCode) -> bool {
    (&*input).get_key_down(keycode)
}

#[no_mangle]
pub unsafe extern "C" fn input_get_keyup(input:*const Input,keycode:KeyCode) -> bool {
    (&*input).get_key_up(keycode)
}

#[no_mangle]
pub unsafe extern "C" fn input_get_mouse_down(input:*const Input,mouse_btn:u32) -> bool {
    
    (&*input).get_mouse_down(mouse_btn.into())
}

#[no_mangle]
pub unsafe extern "C" fn input_get_mouse_up(input:*const Input,mouse_btn:u32) -> bool {
    (&*input).get_mouse_up(mouse_btn.into())
}
