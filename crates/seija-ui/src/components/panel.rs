use std::collections::HashMap;
use seija_core::{log, math::{Vec3, Vec4}};
use bevy_ecs::prelude::Component;
use seija_transform::Transform;
use crate::{mesh2d::{Mesh2D, Vertex2D}};
use super::{rect2d::Rect2D};

#[derive(Component)]
pub struct Panel {
    pub(crate) is_static:bool
}

impl Default for Panel {
    fn default() -> Self {
        Panel { 
            is_static: true
        }
    }

    
}