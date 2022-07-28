mod shadow;
mod shadow_light;
mod shadow_node;
use bevy_ecs::prelude::Component;
pub use shadow_light::{ShadowLight};
pub use shadow::{Shadow};
pub use shadow_node::{ShadowNode};
mod recv_backend;
#[derive(Component)]
pub struct ShadowCamera;