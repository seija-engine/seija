use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct Panel {
    //一经创建不允许修改
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