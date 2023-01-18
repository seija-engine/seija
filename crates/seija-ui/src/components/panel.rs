use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct Panel {
    pub(crate) is_static:bool
}

impl Panel {
    pub fn new(is_static:bool) -> Self {
        Panel { is_static }
    }
}

impl Default for Panel {
    fn default() -> Self {
        Panel { 
            is_static: true
        }
    }

    
}