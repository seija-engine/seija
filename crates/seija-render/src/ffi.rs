use seija_app::App;

use crate::{camera::camera::Camera, RenderModule};

#[no_mangle]
pub unsafe fn render_create_camera()  {
    Camera::default();
}

#[no_mangle]
pub unsafe fn render_add_module(app_ptr:*mut App) {
    let render_module = RenderModule::default();
    (&mut *app_ptr).add_module(render_module);
}