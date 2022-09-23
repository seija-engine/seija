use bevy_ecs::{prelude::Entity, world::World};
use seija_app::App;

use crate::{Transform,TransformModule};

#[no_mangle]
pub unsafe extern "C" fn tranrform_add_module(app_ptr: *mut App) {
    (&mut *app_ptr).add_module(TransformModule);
}

#[no_mangle]
pub unsafe extern "C" fn transform_world_entity_get(world: *mut World, eid: u32) -> *mut Transform {
    let world_mut = &mut *world;
    let t = world_mut
        .get_entity_mut(Entity::from_raw(eid))
        .and_then(|v| v.get_unchecked_mut::<Transform>());
    t.map(|mut t| t.as_mut() as *mut Transform)
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn transform_world_entity_add(world:*mut World,eid:u32,t: *const Transform) {
    let t = (&*t).clone();
    let e = Entity::from_raw(eid);
    let world_mut = &mut *world;
    world_mut.entity_mut(e).insert(t);  
}