use bevy_ecs::{entity::Entity, world::World};
use seija_app::App;
use seija_asset::{HandleId, Handle, AssetServer};
use seija_core::{math::{Vec2, Vec3, Vec4}, TypeUuid};
use spritesheet::SpriteSheet;

use crate::{R2DModule, components::{screen_scaler::{ScreenScaler, ScalerMode, ScreenSizeMatchWHInfo}, sprite::Sprite2D}};

#[no_mangle]
pub unsafe extern "C" fn r2d_add_module(app_ptr:&mut App) {
    app_ptr.add_module(R2DModule);
}


#[no_mangle]
pub unsafe extern "C" fn entity_add_screen_scaler(world:&mut World,entity_id:u64,camera_entity_id:u64,mode:i32,ex_arg0:&Vec3) {
    let entity = Entity::from_bits(entity_id);
    let camera_entity = Entity::from_bits(camera_entity_id);
    let scaler_mode = match mode {
        1 => {
            ScalerMode::ScreenSizeMatchWH(ScreenSizeMatchWHInfo {
                design_size:Vec2::new(ex_arg0.x, ex_arg0.y),
                wh_rate:ex_arg0.z
            })
        } 
        _ => ScalerMode::ConstantPixelSize,
    };
    let scaler = ScreenScaler { mode:scaler_mode, camera_entity:Some(camera_entity) };
    world.entity_mut(entity).insert(scaler);
}

#[no_mangle]
pub unsafe extern "C" fn entity_add_sprite_2d(world:&mut World,atlas_id: u64,entity_id:u64, index: i32,color:&Vec4) {
    let handle_id = HandleId::new(SpriteSheet::TYPE_UUID, atlas_id);
    let handle = if index >= 0 {
        let ref_sender = world.get_resource::<AssetServer>().clone().unwrap().get_ref_sender();
        Some(Handle::<SpriteSheet>::strong(handle_id, ref_sender))
    } else {
        None
    };
    let entity = Entity::from_bits(entity_id);
    let sprite2d = Sprite2D::simple(handle, index as usize, color.clone());
    world.entity_mut(entity).insert(sprite2d);
}