mod resource;
mod mesh;
mod texture;
mod cube_map;
pub mod shape;
pub use texture::{Texture,update_texture_system,ImageInfo,read_image_info,load_image_info};
pub use cube_map::{CubeMapBuilder};
pub use mesh::{Mesh,update_mesh_system,VertexAttributeValues,Indices,MeshAttributeType};
pub use  resource::{RenderResources,RenderResourceId,BufferId,TextureId};

use seija_app::{App};
use seija_asset::{AddAsset};
use seija_core::{AddCore};

pub(crate) fn init_resource(app:&mut App) {
    app.add_asset::<Mesh>();
    app.add_event::<Mesh>();
    
    app.add_asset::<Texture>();
    app.add_event::<Texture>();
}