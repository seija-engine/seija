use std::{rc::Rc, sync::Arc};

use seija_asset::Handle;
use seija_render::resource::{Mesh, Texture};

#[derive(Debug)]
pub struct GltfAsset {
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