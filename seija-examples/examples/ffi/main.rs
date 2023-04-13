use std::sync::Arc;

use seija_app::App;
use seija_asset::AssetModule;
use seija_core::CoreModule;
use seija_input::InputModule;
use seija_render::{RenderModule, RenderConfig};
use seija_winit::WinitModule;


pub fn main() {
    let mut app = App::new();
    app.add_module(CoreModule);
    app.add_module(AssetModule(std::env::current_dir().unwrap().join("res").into()));
    app.add_module(WinitModule::default());
    app.add_module(InputModule);
    
    app.add_module(RenderModule(Arc::new(RenderConfig {
        config_path: "./res/config.json".into(),
        script_path: "./res/script.lua".into(),
        render_lib_paths:vec!["./res/render_libs/".into()],
        ..Default::default()
    })));
    app.start();

    app.run();
}