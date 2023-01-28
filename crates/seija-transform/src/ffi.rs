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
    if let Some(mut entity) = world_mut.get_entity_mut(Entity::from_raw(eid)) {
        let trans = entity.get_mut::<Transform>();
        if let Some(mut trans)  = trans {
            trans.as_mut() as *mut Transform
        } else {
            std::ptr::null_mut()
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn transform_world_entity_add(world:*mut World,eid:u32,t: *const Transform) {
    let t = (&*t).clone();
    let e = Entity::from_raw(eid);
    let world_mut = &mut *world;
    world_mut.entity_mut(e).insert(t);  
}


#[no_mangle]
pub unsafe extern "C" fn tranrform_add(world:*mut World,eid:u32) -> *mut Transform {
    let world_mut = &mut *world;
    let entity = Entity::from_raw(eid);
    let mut entity_mut = world_mut.entity_mut(entity);
    entity_mut.insert(Transform::default());
    if let Some(mut t) = entity_mut.get_mut::<Transform>() {
        t.as_mut() as *mut Transform
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn tranrform_debug_log(world:*mut World,eid:u32) {
    let world_mut = &mut *world;
    let entity = Entity::from_raw(eid);
    let entity_ref = world_mut.entity(entity);
    let t = entity_ref.get::<Transform>();
    println!("{:?}",&t);
}