use bevy_ecs::prelude::Component;
use seija_app::{IModule, App};

use crate::mesh2d::Mesh2D;

use self::rect2d::Rect2D;

pub mod sprite;
pub mod panel;
pub mod rect2d;
mod image_info;


#[derive(Component)]
pub struct ElementTrack;

pub trait IBuildMesh2D {
   fn build(&self,rect2d:&Rect2D) -> Mesh2D;
}