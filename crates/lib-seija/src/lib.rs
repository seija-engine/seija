use seija_app::App;

#[no_mangle]
pub extern "C" fn new_app() -> *mut u8 {
    let app = App::new();
    Box::into_raw(Box::new(app)) as *mut u8
}

#[no_mangle]
pub unsafe extern "C"  fn app_set_fps(app_ptr:*mut u8,fps:u32) {
    let mut_app = &mut *(app_ptr as *mut App);
    mut_app.set_fps(fps);
}

#[no_mangle]
pub unsafe extern "C" fn app_run(app_ptr:*mut u8)  {
    let boxed_app = Box::from_raw(app_ptr as *mut App);
    boxed_app.run()
}
