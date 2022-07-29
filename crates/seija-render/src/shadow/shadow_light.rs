use bevy_ecs::prelude::Component;

#[derive(Component,Default)]
pub struct ShadowLight {
   pub bias:f32,
   pub strength:f32
}