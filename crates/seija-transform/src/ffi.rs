use bevy_ecs::{world::{World, EntityMut}, prelude::Entity};
use seija_app::App;
use crate::{TransformModule, Transform, TransformMatrix};

#[no_mangle]
pub unsafe extern "C" fn transform_add_module(app_ptr: *mut App) {
    (&mut *app_ptr).add_module(TransformModule);
}

#[no_mangle]
pub unsafe extern "C" fn transform_add(world: *mut World, entity_id:u64,t: *const TransformMatrix) {
    let world_mut = &mut *world;
    let local_t = (&*t).clone();
    let mut t = Transform::default();
    t.local = local_t;
    let entity = Entity::from_bits(entity_id);
    world_mut.entity_mut(entity).insert(t);  
}

#[no_mangle]
pub unsafe extern "C" fn transform_debug_log(world:*mut World,eid:u64) {
    let world_mut = &mut *world;
    let entity = Entity::from_bits(eid);
    let entity_ref = world_mut.entity(entity);
    let t = entity_ref.get::<Transform>();
    println!("{:?}",&t);
    
}

/*
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
*/