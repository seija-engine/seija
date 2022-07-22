mod shadow;
mod shadow_light;
mod shadow_node;
use bevy_ecs::prelude::Component;
pub use shadow_light::{ShadowLight};
pub use shadow::{Shadow};
pub use shadow_node::{ShadowNode};

#[derive(Component)]
pub struct ShadowCamera;