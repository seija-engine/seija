use seija_app::{ecs::world::World, App};
use seija_asset::{Assets, HandleId};
use seija_core::{TypeUuid, smol_str::SmolStr};

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


pub struct SpriteListGetFFI {
   pub info_list:Vec<(i32,SmolStr)>
}

#[no_mangle]
pub unsafe extern "C" fn spritesheet_begin_read(sheet:&SpriteSheet,count:&mut i32,name_max:&mut i32) -> *mut SpriteListGetFFI {
    let mut ret_list = vec![];
    let mut max_name_char = 0;
    for kv in sheet.name_dict.iter() {
        ret_list.push((*kv.1 as i32,kv.0.clone()));
        max_name_char = max_name_char.max(kv.0.len());
    }
    *count = ret_list.len() as i32;
    *name_max = max_name_char as i32;
    Box::into_raw(Box::new(SpriteListGetFFI{info_list:ret_list}))
}

#[no_mangle]
pub unsafe extern "C" fn spritesheet_get_info(sheet:&mut SpriteListGetFFI,index:i32,out_index:&mut i32,out_name:*mut i8) -> bool {
    if let Some(info) = sheet.info_list.get(index as usize) {
        *out_index = info.0;
        let c_string = std::ffi::CString::new(info.1.as_str()).unwrap();
        let c_str = c_string.as_c_str();
        std::ptr::copy_nonoverlapping(c_str.as_ptr(),out_name,c_str.to_bytes().len());
        return true;
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn spritesheet_end_read(data:*mut SpriteListGetFFI) {
   let _ = Box::from_raw(data);
}