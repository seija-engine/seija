use bevy_ecs::{prelude::Entity, world::World};
use seija_app::App;
use seija_asset::{AssetServer, Handle, HandleId};
use seija_core::{math::Vec4, TypeUuid};
use seija_render::RenderConfig;
use spritesheet::SpriteSheet;

use crate::{
    components::{canvas::Canvas, rect2d::Rect2D, sprite::Sprite, ui_canvas::UICanvas},
    event::UIEventSystem,
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
