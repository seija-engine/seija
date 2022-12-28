use std::collections::HashMap;
use seija_core::{log, math::{Vec3, Vec4}};
use bevy_ecs::prelude::Component;
use seija_transform::Transform;
use crate::{mesh2d::{Mesh2D, Vertex2D}};
use super::{rect2d::Rect2D};

#[derive(Component)]
pub struct Panel {
    pub is_static:bool,
    pub(crate) child_meshs:HashMap<u32,Mesh2D>
}

impl Default for Panel {
    fn default() -> Self {
        Panel { 
            is_static: true,
            child_meshs:HashMap::default() 
        }
    }
}


impl Panel {
    pub fn build_mesh(&self,_t:&Transform,_rect2d:&Rect2D) -> Option<Mesh2D> {
        let mut points:Vec<Vertex2D> = vec![];
        let mut indexs:Vec<u32> = vec![];
        let mut index_offset = 0u32;
        for child_mesh in self.child_meshs.values() {
            points.extend(child_mesh.points.iter());
            indexs.extend(child_mesh.indexs.iter().map(|v| v + index_offset));
            index_offset += child_mesh.points.len() as u32;
        }

        if points.is_empty() {
            return None;
        }
        //TODO 考虑把颜色值放到顶点上
        Some(Mesh2D {points,indexs,color:Vec4::ONE })
    }
}