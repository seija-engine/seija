use std::sync::Arc;

use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct EInfo {
    pub name:Option<String>,
    pub layer:u32,
    pub tag:Option<Arc<String>>
}