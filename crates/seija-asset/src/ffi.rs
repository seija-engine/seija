use std::{ffi::CStr, path::PathBuf};

use bevy_ecs::world::World;
use seija_app::App;
use seija_core::ResultExt;
use uuid::Uuid;
use crate::{AssetModule, AssetServer, HandleId, HandleUntyped};

#[no_mangle]
pub unsafe extern "C" fn asset_add_module(app_ptr:&mut App,path:*mut i8) {
    let path_str = CStr::from_ptr(path).to_str().log_err().unwrap_or_default();
    let pathbuf = PathBuf::from(path_str);
    println!("add asset module:{:?}",&pathbuf);
    app_ptr.add_module(AssetModule(pathbuf));
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

#[no_mangle]
pub unsafe extern "C" fn asset_get_handle(world:&mut World,path:*const i8,is_weak:bool,handle_id:&mut u64,ta:&mut u64,tb:&mut u64) -> bool {
    let server = world.get_resource::<AssetServer>().unwrap();
    let str_path = std::ffi::CStr::from_ptr(path).to_str().unwrap_or_default();
    if let Some(info) = server.get_asset(str_path) {
        let handle:HandleUntyped = if is_weak { info.make_weak_handle() } else { info.make_handle() };
        *handle_id = handle.id.id;
        let (a,b) = uuid_to_u64(&handle.id.typ);
        *ta = a;
        *tb = b;
        return true;
    } else {
        return false;
    }
}

#[no_mangle]
pub unsafe extern "C" fn asset_load_sync(world:&mut World,path:*const i8,ta:u64,tb:u64,id:&mut u64) -> bool {
    let str_path = std::ffi::CStr::from_ptr(path).to_str().unwrap_or_default();
    let server = world.get_resource_mut::<AssetServer>().unwrap().clone();
    let uuid = uuid_from_u64(ta,tb);
    let asset = server.load_sync_untyped(world,&uuid, str_path, None);
    match asset {
        Ok(mut handle) => {
            handle.forget();
            *id = handle.id.id;
            return true;
        },
        Err(err) => {
            log::error!("load sync error:{:?}",err);
            return false;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn asset_unload(world:&mut World,id:u64,ta:u64,tb:u64)  {
    let uuid = uuid_from_u64(ta,tb);
    let handle_id = HandleId::new(uuid,id);
    let server = world.get_resource_mut::<AssetServer>().unwrap().clone();
    server.unload(handle_id);
}

#[no_mangle]
pub unsafe extern "C" fn string_to_uuid(str:*const i8,ta:&mut u64,tb:&mut u64) -> bool {
    let str = std::ffi::CStr::from_ptr(str).to_str().unwrap_or_default();
    match Uuid::parse_str(str) {
        Ok(uuid) => {
            let (a,b) = uuid_to_u64(&uuid);
            *ta = a;
            *tb = b;
            return true;
        },
        Err(err) => {
            log::error!("parse uuid error:{:?}",err);
            return false;
        }
    }
}

fn uuid_to_u64(uuid:&Uuid) -> (u64,u64) {
    let bytes = uuid.as_bytes();
    let mut a = 0u64;
    let mut b = 0u64;
    for i in 0..8 {
        a |= (bytes[i] as u64) << (i * 8);
    }
    for i in 8..16 {
        b |= (bytes[i] as u64) << ((i - 8) * 8);
    }
    (a,b)
}

fn uuid_from_u64(a:u64,b:u64) -> Uuid {
    let mut bytes = [0u8;16];
    for i in 0..8 {
        bytes[i] = ((a >> (i * 8)) & 0xff) as u8;
    }
    for i in 8..16 {
        bytes[i] = ((b >> ((i - 8) * 8)) & 0xff) as u8;
    }
    Uuid::from_bytes(bytes)
}
