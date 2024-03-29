
use std::ffi::CStr;
use std::os::raw::c_char;
use seija_app::App;
use seija_app::ecs::world::World;
use seija_core::window::{WindowConfig, AppWindow};
use crate::WinitModule;

#[no_mangle]
pub unsafe extern "C" fn winit_add_module(app_ptr:*mut App,config_ptr:*mut WindowConfig) {
    let config= Box::from_raw(config_ptr);
    let winit_module = WinitModule(*config);
    (&mut (*app_ptr)).add_module(winit_module);
}

#[no_mangle]
pub unsafe extern "C" fn winit_new_windowconfig() -> *mut WindowConfig {
    let default = WindowConfig::default();
    Box::into_raw(Box::new(default)) 
}

#[no_mangle]
pub unsafe extern "C" fn winit_windowconfig_set_title(config_ptr:*mut WindowConfig,title:*const c_char) {
    let c_str = CStr::from_ptr(title as *const i8).to_str().unwrap_or_default();
    (&mut (*config_ptr)).title = c_str.into();
}

#[no_mangle]
pub unsafe extern "C" fn set_window_fullscreen(world:&World) {
   if let Some(window) = world.get_resource::<AppWindow>() {
    window.set_fullscreen();
   }
}

#[no_mangle]
pub unsafe extern "C" fn set_maximized(world:&World,value:bool) {
    if let Some(window) = world.get_resource::<AppWindow>() {
        window.set_maximized(value);
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_inner_size(world:&World,w:f32,h:f32) {
    if let Some(window) = world.get_resource::<AppWindow>() {
        window.set_inner_size(w,h);
    }
}