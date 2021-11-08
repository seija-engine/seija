mod system;
mod light;
use seija_app::App;
pub use system::LightState;

use self::light::AmbientLight;

pub fn init_light(app:&mut App) {
    app.add_resource(AmbientLight::default());
}