
pub mod asset;
pub mod loader;
use asset::{GltfAsset, GltfMaterial};
use bevy_ecs::prelude::{Commands, Entity};
pub use gltf;
use loader::GLTFLoader;


use seija_app::{IModule, App};
use seija_asset::{Handle, AddAsset};
use seija_core::TypeUuid;
use seija_render::material::{Material};
use seija_transform::{BuildChildren, Transform};

pub struct GLTFModule;

impl IModule for GLTFModule {
    fn init(&mut self,app:&mut App) {
        app.add_asset::<GltfAsset>();
        app.add_asset_loader(GltfAsset::TYPE_UUID, GLTFLoader);
    }
}

type ImportData = (gltf::Document, Vec<gltf::buffer::Data>, Vec<gltf::image::Data>);

pub fn create_gltf<T>(asset:&GltfAsset,commands:&mut Commands,mat_fn:&T) -> Entity  
             where T: Fn(&GltfMaterial) -> Option<Handle<Material>> {
    let mut mesh_list:Vec<Entity> = vec![];
    for mesh in asset.meshs.iter() {
        let mesh_mat = asset.nodes[mesh.node_index].transform.global();
        for primitive in mesh.primitives.iter() {
            let mut mesh_render = commands.spawn();
            mesh_render.insert(primitive.mesh.clone());
            mesh_render.insert(Transform::from_t_matrix(mesh_mat.clone()));
            if let Some(mat) = primitive.material.as_ref().and_then(|v| mat_fn(&v)) {
                mesh_render.insert(mat);
            }
            mesh_list.push(mesh_render.id());
        }
    }
    let mut root = commands.spawn();
    root.insert(Transform::default());
    root.add_children(&mesh_list);
    root.id()
}