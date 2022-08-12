use std::{sync::Arc};
use seija_core::{TypeUuid,uuid::Uuid};
use seija_skeleton3d::{Skeleton, AnimationSet, Skin};
use seija_transform::{Transform};
use seija_asset::Handle;
use seija_render::{camera::camera::Projection, resource::{Mesh, Texture}};

pub type NodeIndex = usize;
pub type MeshIndex = usize;

#[derive(Debug,Default,TypeUuid)]
#[uuid = "9fb83fbe-b850-42e0-a58c-53da87bbbb05"]
pub struct GltfAsset {
    pub scenes:Vec<GltfScene>,
    pub meshs:Vec<GltfMesh>,
    pub textures:Vec<Handle<Texture>>,
    pub materials:Vec<Arc<GltfMaterial>>,
    pub nodes:Vec<GltfNode>,
    pub skeleton:Option<Handle<Skeleton>>,
    pub skins:Option<Handle<Skin>>,
    pub anims:Option<Handle<AnimationSet>>
}

impl GltfAsset {
    pub fn first_gltf_mesh(&self) -> Option<&GltfMesh> {
        self.meshs.first()
    }

    pub fn first_mesh(&self) -> Option<Handle<Mesh>> {
        self.first_gltf_mesh().map(|v| v.primitives[0].mesh.clone())
    }

}

#[derive(Debug)]
pub struct GltfScene {
    pub nodes:Vec<NodeIndex>
}

#[derive(Debug)]
pub struct GltfNode {
    pub camera:Option<GltfCamera>,
    pub children:Vec<NodeIndex>,
    pub mesh:Option<MeshIndex>,
    pub transform:Transform
}

#[derive(Debug)]
pub struct GltfCamera {
   pub projection:Projection
}

#[derive(Debug)]
pub struct GltfMesh {
    pub node_index:usize,
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