mod resource;
mod mesh;
mod texture;
pub mod shape;
pub use texture::{Texture};
pub use mesh::{Mesh,update_mesh_system};
pub use  resource::{RenderResources,RenderResourceId,BufferId};

use seija_app::{App};
use seija_asset::{AddAsset};
use seija_core::{AddCore};

pub(crate) fn init_resource(app:&mut App) {
    app.add_asset::<Mesh>();
    app.add_event::<Mesh>();
}