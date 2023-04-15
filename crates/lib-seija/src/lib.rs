pub use seija_app::ffi::*;
pub use seija_core::ffi::*;
pub use seija_winit::ffi::*;
pub use seija_transform::ffi::*;
pub use seija_input::ffi::*;
pub use seija_asset::ffi::*;
pub use seija_render::ffi::*;
pub use seija_pbr::ffi::*;

#[no_mangle]
pub extern "C" fn get_version() -> usize {
    1111
} 