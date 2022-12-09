use std::collections::HashMap;

use bevy_ecs::prelude::Component;

use crate::mesh2d::Mesh2D;

#[derive(Component)]
pub struct Panel {
    pub is_static:bool,
    pub is_dirty:bool,
    child_meshs:HashMap<u32,Mesh2D>
}

impl Default for Panel {
    fn default() -> Self {
        Panel { 
            is_static: true,
            is_dirty:false,
            child_meshs:HashMap::default() 
        }
    }
}