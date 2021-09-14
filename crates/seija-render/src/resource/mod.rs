mod resource;
mod mesh;
pub mod shape;
use bevy_ecs::prelude::IntoSystem;
pub use mesh::{Mesh};
pub use  resource::{RenderResources,ResourceId,BufferId};

use seija_app::{App};
use seija_asset::{AddAsset};
use seija_core::CoreStage;

pub(crate) fn init_resource(app:&mut App) {
    app.add_asset::<Mesh>();
    app.add_system(CoreStage::PostUpdate, mesh::update_mesh_system.system());
}