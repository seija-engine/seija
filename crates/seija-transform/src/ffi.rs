use bevy_ecs::{world::{World, EntityMut}, prelude::Entity};
use seija_app::App;
use crate::{TransformModule, Transform, TransformMatrix};

#[no_mangle]
pub unsafe extern "C" fn transform_add_module(app_ptr: &mut App) {
    app_ptr.add_module(TransformModule);
}

#[no_mangle]
pub unsafe extern "C" fn transform_add(world: *mut World, entity_id:u64,t: *const TransformMatrix) {
    let world_mut = &mut *world;
    let local_t = (&*t).clone();
    let mut t = Transform::default();
    t.local = local_t;
    let entity = Entity::from_bits(entity_id);
    log::error!("add transform to entity: {:?} with local: {:?}",entity,&t);
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

#[no_mangle]
pub unsafe extern "C" fn transform_mut_view(world:*mut World,entity_id:u64,viewf:extern fn(world:*mut Transform)) {
    let world_mut = &mut *world;
    let entity = Entity::from_bits(entity_id);
    if let Some(mut t) = world_mut.entity_mut(entity).get_mut::<Transform>() {
        viewf(t.as_mut());
    }
}

#[no_mangle]
pub unsafe extern "C" fn transform_get_ptr(world:*mut World,entity_id:u64) -> *mut Transform {
    let world_mut = &mut *world;
    let entity = Entity::from_bits(entity_id);
    if let Some(mut t) = world_mut.entity_mut(entity).get_mut::<Transform>() {
        t.as_mut() as *mut Transform
    } else {
        std::ptr::null_mut()
    }
    
}
