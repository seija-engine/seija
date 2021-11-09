mod system;
mod light;
use seija_app::App;
pub use system::LightState;

pub use light::LightEnv;

pub fn init_light(app:&mut App) {
    app.add_resource(LightEnv::default());
}