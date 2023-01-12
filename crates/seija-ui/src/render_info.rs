use bevy_ecs::{prelude::Entity, system::Commands};
use seija_asset::{Assets, AssetServer};
use seija_render::{resource::Mesh, material::Material};
use seija_transform::Transform;
use crate::mesh2d::Mesh2D;

pub struct PanelInfo {

}

impl PanelInfo {
    pub fn create() -> PanelInfo {
        PanelInfo {  }
    }
}