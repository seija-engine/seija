use std::{sync::Arc};

use bevy_ecs::{prelude::Entity, world::World};
use seija_app::App;
use seija_asset::{uuid_from_u64, Handle, HandleId, AssetServer};
use crate::{camera::camera::{Camera, Projection,Orthographic,Perspective}, RenderModule, RenderConfig, resource::Mesh, material::Material};


#[no_mangle]
pub unsafe fn render_add_module(app_ptr:&mut App,config_ptr:*mut RenderConfig) {
    let config = Box::from_raw(config_ptr);
    let render_module = RenderModule(Arc::new(*config));
    app_ptr.add_module(render_module);
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


#[no_mangle]
pub unsafe fn render_create_camera() -> *mut Camera {
    let camera = Box::new(Camera::default());
    Box::into_raw(camera)
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
    dbg!(perspective);
    dbg!(Perspective::default());
    let projection = Box::new(Projection::Perspective(Default::default()));
    Box::into_raw(projection)
}

#[no_mangle]
pub unsafe fn render_camera_set_path(camera_ptr:*mut Camera,path:*const i8) {
    let camera = &mut *camera_ptr;
    let path = std::ffi::CStr::from_ptr(path).to_str().unwrap_or_default();
    camera.path = path.into();
}

#[no_mangle]
pub unsafe fn render_entity_get_camera(world:&mut World,entity_id:u64) -> *mut Camera {
    let entity = Entity::from_bits(entity_id);
    if let Some(mut camera_ptr) = world.entity_mut(entity).get_mut::<Camera>() {
       return camera_ptr.as_mut() as *mut Camera;
    }
    std::ptr::null_mut()
}

#[no_mangle]
pub unsafe fn render_entity_add_camera(world:&mut World,entity_id:u64,camera_ptr:*mut Camera) {
    let entity = Entity::from_bits(entity_id);
    let camera = Box::from_raw(camera_ptr);
    
    world.entity_mut(entity).insert(*camera);
}

#[no_mangle]
pub unsafe fn render_entity_add_mesh(world:&mut World,entity_id:u64,id:u64,ta:u64,tb:u64) {
    let entity = Entity::from_bits(entity_id);
    let typ_uuid = uuid_from_u64(ta, tb);
    let hid = HandleId::new(typ_uuid, id);
    let sender = world.get_resource::<AssetServer>().unwrap().clone().get_ref_sender();
    let mesh_handle:Handle<Mesh> = Handle::strong(hid,sender);
    world.entity_mut(entity).insert(mesh_handle);
    
}

#[no_mangle]
pub unsafe fn render_entity_add_material(world:&mut World,entity_id:u64,id:u64,ta:u64,tb:u64) {
    let entity = Entity::from_bits(entity_id);
    let typ_uuid = uuid_from_u64(ta, tb);
    let hid = HandleId::new(typ_uuid, id);
    let sender = world.get_resource::<AssetServer>().unwrap().clone().get_ref_sender();
    let mat_handle:Handle<Material> = Handle::strong(hid,sender);
    world.entity_mut(entity).insert(mat_handle);
}