use std::collections::HashMap;
use seija_core::log;
use bevy_ecs::prelude::Component;
use seija_transform::Transform;

use crate::{mesh2d::Mesh2D, types::Rect};

use super::{IBuildMesh2D, rect2d::Rect2D};

#[derive(Component)]
pub struct Panel {
    pub is_static:bool,
    pub is_dirty:bool,
    pub(crate) child_meshs:HashMap<u32,Mesh2D>
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


impl Panel {
    pub fn build_mesh(&self,t:&Transform,rect2d:&Rect2D) -> Option<Mesh2D> {
        
        for child_mesh in self.child_meshs.iter() {
            log::error!("id:{}",child_mesh.0);
        }
        None
    }
}