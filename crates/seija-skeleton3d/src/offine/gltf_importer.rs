pub struct GltfImporter {

}

impl GltfImporter {
    pub fn load<S>(bytes:S)  where S: AsRef<[u8]> {
       let mut import_data = gltf::import_slice(bytes).unwrap();
       
    }

    pub fn import() {

    }
}