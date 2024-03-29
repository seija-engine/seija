use bevy_ecs::{world::World, prelude::Entity};
use seija_app::App;
use seija_core::info::EStateInfo;
use crate::hierarchy::Parent;
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
    world.set_parent(child_entity, Some(cur_entity));
    world.move_child(child_entity,cur_entity,index as usize);
}

#[no_mangle]
pub unsafe extern "C" fn transform_despawn(world:&mut World,entity_id:u64) {
    let entity = Entity::from_bits(entity_id);
    world.delete(entity);
}

#[no_mangle]
pub unsafe extern "C" fn transform_set_active(world:&mut World,entity_id:u64,is_active:bool) {
    let entity = Entity::from_bits(entity_id);
    world.set_active(entity,is_active);
}

#[no_mangle]
pub unsafe extern "C" fn transform_is_active_global(world:&mut World,entity_id:u64) -> bool {
    let entity = Entity::from_bits(entity_id);
    world.entity(entity).get::<EStateInfo>().map(|v| v.is_active_global()).unwrap_or(true)
}

#[no_mangle]
pub unsafe extern "C" fn transform_add_state_info(world:&mut World,entity_id:u64,is_active:bool) {
    let entity = Entity::from_bits(entity_id);
    let mut info = EStateInfo::default();
    info.set_active(is_active);
    info._is_active_global = is_active;
    world.entity_mut(entity).insert(info);
}

#[no_mangle]
pub unsafe extern "C" fn transform_relative_to(world:&mut World,out_ptr:&mut TransformMatrix,child:u64,parent:u64,is_nil_parent:bool) {
    let mut parents = world.query::<&Parent>();
    let mut trans = world.query::<&Transform>();
    let child_entity = Entity::from_bits(child);
    let parent_entity = Entity::from_bits(parent);

    let mut cur_trans = if is_nil_parent {
        TransformMatrix::default()
    } else if let Ok(v) = trans.get(world, child_entity) { 
        v.local.clone()
    } else { TransformMatrix::default() };
    let mut cur_entity = Some(child_entity);
    while let Some(entity) = cur_entity {
        if let Ok(p) = parents.get(world, entity) {
            if p.0 == parent_entity {
                break;
            }
            if let Ok(t) = trans.get(world, p.0) {
                cur_trans = cur_trans.mul_transform(&t.local);
            }
            cur_entity = Some(p.0)
        } else { break; }
    }
    *out_ptr = cur_trans
}

#[no_mangle]
pub unsafe extern "C" fn core_spawn_entity(world:&mut World) -> u64 {
    let new_entity = world.new_empty(true);
    new_entity.to_bits()
}