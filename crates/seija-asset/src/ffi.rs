use std::{ffi::CStr, path::PathBuf};

use bevy_ecs::world::World;
use seija_app::App;
use seija_core::ResultExt;
use crate::{AssetModule, AssetServer};

#[no_mangle]
pub unsafe extern "C" fn asset_add_module(app_ptr:*mut App,path:*mut i8) {
    let path_str = CStr::from_ptr(path).to_str().log_err().unwrap_or_default();
    let pathbuf = PathBuf::from(path_str);
    (&mut *app_ptr).add_module(AssetModule(pathbuf));
}

#[no_mangle]
pub unsafe extern "C" fn asset_world_get_ptr(world_ptr:*mut World) -> *const AssetServer {
    let world = &mut *world_ptr;
    if let Some(asset_server) = world.get_resource::<AssetServer>() {
        asset_server as *const AssetServer
    } else {
        std::ptr::null()
    }
}


#[no_mangle]
pub unsafe extern "C" fn asset_full_path(asset_ptr:*const AssetServer,path:*const i8,out_buffer:*const i8) {
    let asset_server = &*asset_ptr;
    let path = std::ffi::CStr::from_ptr(path).to_str().unwrap_or_default();
    let full_path = asset_server.full_path(path);
    //std::ffi::CStr::from_ptr(full_path.as_ptr() as *const i8);   
}