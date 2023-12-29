use crate::App;

#[no_mangle]
pub extern "C" fn app_new() -> *mut u8 {
    let app = App::new();
    Box::into_raw(Box::new(app)) as *mut u8
}

#[no_mangle]
pub unsafe extern "C" fn app_set_fps(app_ptr:*mut App,fps:u32) {
    let mut_app = &mut *app_ptr;
    mut_app.set_fps(fps);
}

#[no_mangle]
pub unsafe extern "C" fn app_run(app_ptr:*mut App)  {
    let boxed_app = Box::from_raw(app_ptr);
    boxed_app.run()
}

#[no_mangle]
pub unsafe extern "C" fn app_start(app_ptr:*mut App)  {
    let mut_app = &mut *app_ptr;
    mut_app.start()
}

#[no_mangle]
pub unsafe extern "C" fn app_quit(app_ptr:*mut App)  {
    let mut_app = &mut *app_ptr;
    mut_app.quit()
}


