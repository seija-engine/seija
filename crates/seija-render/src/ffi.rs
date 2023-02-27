use seija_app::App;

use crate::{camera::camera::{Camera, Projection,Orthographic,Perspective}, RenderModule, RenderConfig};

#[no_mangle]
pub unsafe fn render_create_camera() -> *mut i8 {
    let camera = Box::new(Camera::default());
    Box::into_raw(camera) as *mut i8
}

#[no_mangle]
pub unsafe fn render_camera_set_projection(camera_ptr:*mut Camera,projection:*mut Projection) {
    let camera = &mut *camera_ptr;
    let projection = &mut *projection;
    camera.projection = *Box::from_raw(projection);
}

#[no_mangle]
pub unsafe fn render_create_ortho_projection(ortho_ptr:*mut Orthographic) -> *mut Projection {
    let ortho = &*ortho_ptr;
    let projection = Box::new(Projection::Ortho(ortho.clone()));
    Box::into_raw(projection)
}

#[no_mangle]
pub unsafe fn render_create_perpective_projection(perspective_ptr:*mut Perspective) -> *mut Projection {
    let perspective = &*perspective_ptr;
    let projection = Box::new(Projection::Perspective(perspective.clone()));
    Box::into_raw(projection)
}

#[no_mangle]
pub unsafe fn render_camera_set_path(camera_ptr:*mut Camera,path:*const i8) {
    let camera = &mut *camera_ptr;
    let path = std::ffi::CStr::from_ptr(path).to_str().unwrap_or_default();
    camera.path = path.into();
}


#[no_mangle]
pub unsafe fn render_add_module(app_ptr:*mut App) {
    let render_module = RenderModule::default();
    (&mut *app_ptr).add_module(render_module);
}

#[no_mangle]
pub unsafe fn render_create_config() -> *mut RenderConfig {
    let config = Box::new(RenderConfig::default());
    Box::into_raw(config)
}

#[no_mangle]
pub unsafe fn render_config_set_config_path(config_ptr:*mut RenderConfig,path:*const i8) {
    let config = &mut *config_ptr;
    let path = std::ffi::CStr::from_ptr(path).to_str().unwrap_or_default();
    config.config_path = path.into();
}

#[no_mangle]
pub unsafe fn render_config_set_script_path(config_ptr:*mut RenderConfig,path:*const i8) {
    let config = &mut *config_ptr;
    let path = std::ffi::CStr::from_ptr(path).to_str().unwrap_or_default();
    config.script_path = path.into();
}

#[no_mangle]
pub unsafe fn render_config_add_render_lib_path(config_ptr:*mut RenderConfig,path:*const i8) {
    let config = &mut *config_ptr;
    let path = std::ffi::CStr::from_ptr(path).to_str().unwrap_or_default();
    config.render_lib_paths.push(path.into());
}