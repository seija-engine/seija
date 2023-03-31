pub use seija_app::ffi::*;
pub use seija_core::ffi::*;
pub use seija_winit::ffi::*;
pub use seija_transform::ffi::*;
pub use seija_input::ffi::*;
pub use seija_asset::ffi::*;

#[no_mangle]
pub extern "C" fn get_version() -> usize {
    1111
}