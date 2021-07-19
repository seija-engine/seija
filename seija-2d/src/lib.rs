pub mod elems;
use bevy_app::{AppBuilder, Plugin};
use elems::sprite_system;
use bevy_ecs::prelude::*;
use sprite_system::sprite_mesh_system;
#[derive(Default)]
pub struct Seija2DPlugin();

impl Plugin for Seija2DPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(sprite_mesh_system.system());
    }
}