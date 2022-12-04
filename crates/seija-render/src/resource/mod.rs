mod resource;
mod mesh;
mod texture;
mod image_info;
mod cube_map;
pub mod shape;
mod loader;
pub use texture::{Texture,TextureDescInfo,TextureType,color_texture,cube_texture,update_texture_system};
pub use image_info::{ImageInfo,read_image_info,load_image_info};
pub use cube_map::{CubeMapBuilder};
pub use mesh::{Mesh,update_mesh_system,VertexAttributeValues,Indices,MeshAttributeType};
pub use  resource::{RenderResources,RenderResourceId,BufferId,TextureId};

use seija_app::{App};
use seija_asset::{AddAsset};

use self::loader::TextureLoader;


pub(crate) fn init_resource(app:&mut App) {
    app.add_asset::<Mesh>();
    app.add_asset::<Texture>();
    app.add_asset_loader::<Texture,TextureLoader>();
}