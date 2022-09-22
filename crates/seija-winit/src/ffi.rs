use seija_app::App;
use seija_core::window::WindowConfig;
use crate::WinitModule;

#[no_mangle]
pub unsafe extern "C" fn winit_add_module(app_ptr:*mut App,config_ptr:*mut WindowConfig) {
    let config= Box::from_raw(config_ptr);
    let winit_module = WinitModule(*config);
    (&mut (*app_ptr)).add_module(winit_module);
}

