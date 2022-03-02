use seija_asset::Handle;

use crate::{material::Material, resource::Mesh};

#[derive(Debug)]
pub struct MeshRender {
    pub mesh:Handle<Mesh>,
    pub material:Handle<Material>
}