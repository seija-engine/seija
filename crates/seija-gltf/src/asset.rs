use std::{rc::Rc, sync::Arc};
use seija_transform::{Transform};
use seija_asset::Handle;
use seija_render::resource::{Mesh, Texture};

pub type NodeIndex = usize;
pub type MeshIndex = usize;

#[derive(Debug)]
pub struct GltfAsset {
    pub scenes:Vec<GltfScene>,
    pub meshs:Vec<GltfMesh>,
    pub textures:Vec<Handle<Texture>>,
    pub materials:Vec<Arc<GltfMaterial>>
}

impl GltfAsset {
    pub fn first_mesh(&self) -> &GltfMesh {
        self.meshs.first().unwrap()
    }
}

#[derive(Debug)]
pub struct GltfScene {
    pub nodes:Vec<NodeIndex>
}

#[derive(Debug)]
pub struct GltfNode {
    pub children:Vec<NodeIndex>,
    pub mesh:Option<MeshIndex>,
    pub transform:Transform
}

#[derive(Debug)]
pub struct GltfMesh {
    pub primitives: Vec<GltfPrimitive>,
}

#[derive(Debug)]
pub struct GltfPrimitive {
    pub mesh: Handle<Mesh>,
    pub material:Option<Arc<GltfMaterial>>
}

#[derive(Debug)]
pub struct GltfMaterial {
    pub base_color:[f32;4],
    pub base_color_texture:Option<Handle<Texture>>
}