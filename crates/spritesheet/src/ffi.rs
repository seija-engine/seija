use seija_app::{ecs::world::World, App};
use seija_asset::{Assets, HandleId};
use seija_core::TypeUuid;

use crate::{SpriteSheet, SpriteSheetModule};

#[no_mangle]
pub unsafe extern "C" fn spritesheet_add_module(app: &mut App) {
    app.add_module(SpriteSheetModule);
}

#[no_mangle]
pub unsafe extern "C" fn spritesheet_asset_get(world: &mut World, id: u64) -> *mut SpriteSheet {
    let mut sheets = world.get_resource_mut::<Assets<SpriteSheet>>().unwrap();
    let h_sheet = HandleId::new(SpriteSheet::TYPE_UUID, id);
    if let Some(sheet) = sheets.get_mut(&h_sheet) {
        return sheet as *mut SpriteSheet;
    }
    return std::ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn spritesheet_get_index(sheet:&mut SpriteSheet,name:*mut i8) -> i32 {
    let str_name = std::ffi::CStr::from_ptr(name).to_str().unwrap_or_default();
    if let Some(index) = sheet.get_index(str_name) {
       return index as i32;
    }
    return -1;
}