
#[derive(Debug)]
pub enum GltfError {
    UnsupportedPrimitive(gltf::mesh::Mode),
    LoadGltfError(gltf::Error)
}