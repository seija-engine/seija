use seija_app::App;

use crate::CoreModule;

#[no_mangle]
pub unsafe extern "C" fn core_add_module(app_ptr:*mut u8) {
    let mut_app = &mut *(app_ptr as *mut App);
    mut_app.add_module(CoreModule);
}
