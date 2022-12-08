use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct Panel {
    pub is_static:bool,
    pub is_dirty:bool
}

impl Default for Panel {
    fn default() -> Self {
        Panel { is_static: true,is_dirty:false }
    }
}