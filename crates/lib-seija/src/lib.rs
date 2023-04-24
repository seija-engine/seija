pub use seija_app::ffi::*;
pub use seija_core::ffi::*;
pub use seija_winit::ffi::*;
pub use seija_transform::ffi::*;
pub use seija_input::ffi::*;
pub use seija_asset::ffi::*;
pub use seija_render::ffi::*;
pub use seija_pbr::ffi::*;
pub use spritesheet::ffi::*;
pub use seija_ui::ffi::*;

const OUT_STRING:&str = "Aaa_汉字";
#[no_mangle]
pub extern "C" fn debug_cstring(length:&mut i32) -> *mut i8 {
    let ptr = OUT_STRING.as_ptr() as *mut i8;
    *length = OUT_STRING.len() as i32;
    ptr
} 