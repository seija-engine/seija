use bevy_ecs::{world::{World}, prelude::Entity};
use seija_app::App;
use crate::{TransformModule, Transform, TransformMatrix};
use crate::events::WorldEntityEx;
#[no_mangle]
pub unsafe extern "C" fn transform_add_module(app_ptr: &mut App) {
    app_ptr.add_module(TransformModule);
}

#[no_mangle]
pub unsafe extern "C" fn transform_add(world: *mut World, entity_id:u64,t: *const TransformMatrix) {
    let world_mut = &mut *world;
    let local_t = (&*t).clone();
    //log::error!("add:{:?}={:?}",entity_id,&local_t);
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

#[no_mangle]
pub unsafe extern "C" fn transform_set_parent(world:&mut World,entity_id:u64,parent_id:u64,is_null:bool) {
    let entity = Entity::from_bits(entity_id);
    let parent_id = Entity::from_bits(parent_id);
    world.set_parent(entity, if is_null { None } else {Some(parent_id)});
}

#[no_mangle]
pub unsafe extern "C" fn transform_add_child_index(world:&mut World,entity_id:u64,child_id:u64,index:i32) {
    let cur_entity = Entity::from_bits(entity_id);
    let child_entity = Entity::from_bits(child_id);
    //println!("transform_add_child_index:{:?} {:?} {}",&cur_entity,&child_entity,index);
    //world.entity_mut(cur_entity).add_child_index(child_entity, index as usize);
    world.move_child(cur_entity,child_entity,index as usize);
}

#[no_mangle]
pub unsafe extern "C" fn transform_despawn(world:&mut World,entity_id:u64) {
    let entity = Entity::from_bits(entity_id);
    world.delete(entity);
}