use bevy_ecs::{prelude::{Entity, Events}, world::World, event::ManualEventReader};
use seija_app::App;
use seija_asset::{AssetServer, Handle, HandleId};
use seija_core::{math::Vec4, TypeUuid, smol_str::SmolStr};
use seija_render::RenderConfig;
use spritesheet::SpriteSheet;

use crate::{
    components::{canvas::Canvas, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    event::{UIEventSystem, EventNode, UIEvent},
    types::Thickness,
    update_ui_render, UIModule,
};

#[no_mangle]
pub unsafe extern "C" fn ui_add_module(app_ptr: &mut App) {
    app_ptr.add_module(UIModule);
}

#[no_mangle]
pub unsafe extern "C" fn render_config_set_ui(config: &mut RenderConfig) {
    config.pre_render_updates = vec![update_ui_render];
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_ui_canvas(world: &mut World, entity_id: u64, typ: u8) {
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(UICanvas {
        scale_mode: typ.into(),
    });
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_ui_system(world: &mut World, entity_id: u64) {
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(UIEventSystem::default());
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_rect2d(world: &mut World, entity_id: u64, rect: &mut Rect2D) {
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(rect.clone());
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_canvas(world: &mut World, entity_id: u64) {
    let entity = Entity::from_bits(entity_id);
    world.entity_mut(entity).insert(Canvas::default());
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_sprite_simple(
    world: &mut World,
    entity_id: u64,
    index: i32,
    atlas_id: u64,
    color: &mut Vec4,
) {
    let handle_id = HandleId::new(SpriteSheet::TYPE_UUID, atlas_id);
    let ref_sender = world
        .get_resource::<AssetServer>()
        .clone()
        .unwrap()
        .get_ref_sender();
    let handle = Handle::strong(handle_id, ref_sender);
    let entity = Entity::from_bits(entity_id);
    let sprite = Sprite::simple(index as usize, Some(handle), color.clone());
    world.entity_mut(entity).insert(sprite);
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_sprite_slice(
    world: &mut World,
    entity_id: u64,
    index: i32,
    atlas_id: u64,
    thickness: &mut Thickness,
    color: &mut Vec4,
) {
    let handle_id = HandleId::new(SpriteSheet::TYPE_UUID, atlas_id);
    let ref_sender = world.get_resource::<AssetServer>().clone().unwrap().get_ref_sender();
    let handle = Handle::strong(handle_id, ref_sender);
    let entity = Entity::from_bits(entity_id);

    let sprite = Sprite::sliced(
        index as usize,
        Some(handle),
        thickness.clone(),
        color.clone(),
    );
    world.entity_mut(entity).insert(sprite);
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_event_node(world: &mut World, entity_id: u64,node:&EventNode,user_key_ptr:*const i8) {
    let entity = Entity::from_bits(entity_id);
    let mut event_node = node.clone();
    
    if !user_key_ptr.is_null() {
        let user_key_str = std::ffi::CStr::from_ptr(user_key_ptr).to_str().unwrap();
        let user_key = Some(SmolStr::new(user_key_str));
        event_node.user_key = user_key;
    } else {
        event_node.user_key = None;
    }
    //log::info!("event_node:{:?}",&event_node);
    world.entity_mut(entity).insert(event_node);
}

#[no_mangle]
pub unsafe extern "C" fn read_ui_events(world: &mut World,f:extern fn(entity:u64,typ:u32,user_key_ptr:*const i8)) {
    let events = world.get_resource_mut::<Events<UIEvent>>().unwrap();
    let mut reader:ManualEventReader<UIEvent> = events.get_reader();
    for event in reader.iter(&events) {
        if let Some(user_key) = &event.user_key {
           //TODO 考虑优化这里
           let c_string = std::ffi::CString::new(user_key.as_str()).unwrap();
           f(event.entity.to_bits(),event.event_type.bits(),c_string.as_ptr());
        } else {
            f(event.entity.to_bits(),event.event_type.bits(),std::ptr::null());
        }
       
    }
}