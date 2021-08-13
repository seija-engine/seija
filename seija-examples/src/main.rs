use std::time::Duration;

use seija_app::App;
use seija_core::CoreModule;

fn main() {
    let mut app = App::new();
    app.add_module(CoreModule);
    app.set_runner(loop_fn);
    app.run();
}

fn loop_fn(mut app:App) {
    loop {
        app.update();
        std::thread::sleep(Duration::from_millis(10));   
    }
}
