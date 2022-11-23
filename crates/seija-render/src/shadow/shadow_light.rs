use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct ShadowLight {
   pub bias:f32,
   pub strength:f32
}

impl Default for ShadowLight {
   fn default() -> Self {
      ShadowLight {
         bias:0.005f32,
         strength:0.6f32
      }
   }
}