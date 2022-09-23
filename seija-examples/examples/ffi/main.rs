use seija_app::App;
use seija_asset::AssetModule;
use seija_core::CoreModule;
use seija_input::InputModule;
use seija_winit::WinitModule;


pub fn main() {
    let mut app = App::new();
    app.add_module(CoreModule);
    app.add_module(AssetModule(std::env::current_dir().unwrap().join("res").into()));
    app.add_module(WinitModule::default());
    app.add_module(InputModule);
    
    app.start();

    app.run();
}