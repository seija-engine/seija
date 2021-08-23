use std::borrow::Borrow;

use seija_app::App;
use seija_core::CoreModule;
use seija_render::{RenderModule};
use seija_winit::WinitModule;
use wgpu_backed::WGPUBackend;
fn main() {
    let mut app = App::new();
    app.add_module(CoreModule);
    app.add_module(WinitModule::default());


    app.add_module(RenderModule(Box::new( WGPUBackend::default())));

    
    app.run();
}

/* 
fn loop_fn(mut app:App) {
    loop {
        app.update();
        std::thread::sleep(Duration::from_millis(10));   
    }
}*/