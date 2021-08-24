use std::borrow::Borrow;

use seija_app::App;
use seija_core::CoreModule;
use seija_render::{RenderModule};
use seija_winit::WinitModule;
fn main() {
    let mut app = App::new();
    app.add_module(CoreModule);
    app.add_module(WinitModule::default());
    app.add_module(RenderModule);
    app.run();
}
