use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct Shadow {
    pub cast_shadow:bool,
    pub receive_shadow:bool
}

impl Default for Shadow {
    fn default() -> Self {
        Shadow { cast_shadow: true, receive_shadow: true }
    }
}