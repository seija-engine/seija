mod light;
use seija_app::App;

pub use light::{LightEnv,LightType,Light};

pub fn init_light(app:&mut App) {
    app.add_resource(LightEnv::default());
}