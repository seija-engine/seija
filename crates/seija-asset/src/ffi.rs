use std::{ffi::CStr, path::PathBuf};

use seija_app::App;
use seija_core::LogResult;

use crate::AssetModule;
#[no_mangle]
pub unsafe extern "C" fn assert_add_module(app_ptr:*mut App,path:*mut i8) {
    let path_str = CStr::from_ptr(path).to_str().log_err().unwrap_or_default();
    let pathbuf = PathBuf::from(path_str);
    (&mut *app_ptr).add_module(AssetModule(pathbuf));
}
