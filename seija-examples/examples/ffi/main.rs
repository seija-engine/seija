use seija_app::App;
use seija_core::CoreModule;
use seija_winit::WinitModule;


pub fn main() {
    let mut app = App::new();
    app.add_module(CoreModule);
    app.add_module(WinitModule::default());

    app.start();

    app.run();
}