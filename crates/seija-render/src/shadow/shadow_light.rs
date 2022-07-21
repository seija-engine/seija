use bevy_ecs::prelude::Component;

#[derive(Component,Default)]
pub struct ShadowLight {
    bias:f32,
    strength:f32
}