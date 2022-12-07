use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct Panel {
    pub is_static:bool
}

impl Default for Panel {
    fn default() -> Self {
        Panel { is_static: true}
    }
}
/*
Panel

Sprite
Transform
Rect2D
ElementTrack
*/