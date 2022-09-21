use bevy_ecs::prelude::World;
use seija_app::App;

use crate::{CoreModule, time::Time};

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