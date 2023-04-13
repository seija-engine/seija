use std::{ffi::c_char, str::FromStr};

use bevy_ecs::{prelude::World};
use log::Level;
use seija_app::App;
use seija_app::ecs::prelude::*;
use crate::{CoreModule, time::Time, CoreStage, StartupStage};

#[no_mangle]
pub unsafe extern "C" fn core_add_module(app_ptr:*mut u8) {
    let mut_app = &mut *(app_ptr as *mut App);
    mut_app.add_module(CoreModule);
}

#[no_mangle]
pub unsafe extern "C" fn core_world_get_time(world_ptr:*mut u8) -> *const u8 {
    let world_ptr = & *(world_ptr as *mut World);
    if let Some(time) = world_ptr.get_resource::<Time>() {
        let ptr=  time as *const Time;
        return ptr as *const u8;
    }
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn core_time_get_frame(time_ptr:* const u8) -> u64 {
    let time_ref = &*(time_ptr as *const Time);
    time_ref.frame()
}

#[no_mangle]
pub unsafe extern "C" fn core_time_get_delta_seconds(time_ptr:* const u8) -> f32 {
    let time_ref = &*(time_ptr as *const Time);
    time_ref.delta_seconds()
}

#[no_mangle]
pub unsafe extern "C" fn core_spawn_entity(world_ptr:*mut u8) -> u64 {
    let world_ptr = &mut *(world_ptr as *mut World);
   
    world_ptr.spawn_empty().id().to_bits()
}

#[no_mangle]
pub unsafe extern "C" fn init_log(level:*const c_char) {
    let str_level = std::ffi::CStr::from_ptr(level).to_str().unwrap();
    let level:Level = Level::from_str(str_level).unwrap();
    simple_logger::init_with_level(level).unwrap();
}
/*
#[repr(C)]
pub struct FFIEntityMut {
    entity:Entity,
    location:EntityLocation,
}

#[no_mangle]
pub unsafe extern "C" fn core_spawn_empty_entity(world_ptr:*mut u8,entity_mut:*mut FFIEntityMut) {
    let world_ptr = &mut *(world_ptr as *mut World);
    let entity_mut_ptr = &mut *entity_mut;
    let emut = world_ptr.spawn_empty();
    entity_mut_ptr.entity = emut.id();
    entity_mut_ptr.location = emut.location();
   
}*/


type WorldFN = extern fn(world:*mut World);

#[derive(Resource)]
struct OnStartFN(WorldFN);

#[no_mangle]
pub unsafe extern "C" fn app_set_on_start(app_ptr:*mut App,start_fn:WorldFN) {
    let mut_app = &mut *app_ptr;
    mut_app.world.insert_resource(OnStartFN(start_fn));
    mut_app.add_system2(CoreStage::Startup,StartupStage::Startup,on_start_system);
}

fn on_start_system(world:&mut World) {
    if let Some(f) =  world.get_resource::<OnStartFN>() {
         f.0(world);
    }
}

 #[derive(Resource)]
struct OnUpdateFN(WorldFN);

#[no_mangle]
pub unsafe extern "C" fn app_set_on_update(app_ptr:*mut App,update_fn:WorldFN) {
    let mut_app = &mut *app_ptr;
    mut_app.world.insert_resource(OnUpdateFN(update_fn));
    mut_app.add_system(CoreStage::Update,on_update_system);
}

fn on_update_system(world:&mut World) {
    if let Some(f) =  world.get_resource::<OnUpdateFN>() {
         f.0(world);
    }
 }