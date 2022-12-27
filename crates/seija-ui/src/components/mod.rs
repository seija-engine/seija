use bevy_ecs::prelude::Component;
use seija_core::math::Mat4;
use crate::{mesh2d::Mesh2D, types::Rect};
use self::rect2d::Rect2D;
pub mod sprite;
pub mod panel;
pub mod rect2d;
mod image_info;
pub mod ui_canvas;


#[derive(Component)]
pub struct ElementTrack;

pub trait IBuildMesh2D {
   fn build(&self,rect2d:&Rect2D,uv:Rect<f32>,mat:&Mat4,raw_size:&Rect<u32>) -> Mesh2D;
}