use seija_asset::Handle;
use seija_render::resource::Mesh;

#[derive(Debug)]
pub struct GltfAsset {
    pub meshs:Vec<GltfMesh>
}

#[derive(Debug)]
pub struct GltfMesh {
    pub primitives: Vec<GltfPrimitive>,
}

#[derive(Debug)]
pub struct GltfPrimitive {
    pub mesh: Handle<Mesh>
}