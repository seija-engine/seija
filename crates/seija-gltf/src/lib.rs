mod import;
pub mod asset;
pub mod loader;

use asset::{GltfAsset, GltfMaterial};
use bevy_ecs::prelude::{Commands, Entity};
pub use gltf;

use loader::GLTFLoader;
use seija_app::{IModule, App};
use seija_asset::{Handle, AddAsset};

use seija_render::{material::{Material}, shadow::Shadow};
use seija_transform::{BuildChildren, Transform};

pub struct GLTFModule;

impl IModule for GLTFModule {
    fn init(&mut self,app:&mut App) {
        app.add_asset::<GltfAsset>();
        app.add_asset_loader::<GltfAsset,GLTFLoader>();
    }
}

pub fn create_gltf<T>(asset:&GltfAsset,commands:&mut Commands,mut mat_fn:T) -> Entity  
             where T: FnMut(&GltfMaterial) -> Option<Handle<Material>> {
    let mut mesh_list:Vec<Entity> = vec![];
    for mesh in asset.meshs.iter() {
        let mesh_mat = asset.nodes[mesh.node_index].transform.global();
        for primitive in mesh.primitives.iter() {
            let mut mesh_render = commands.spawn_empty();
            mesh_render.insert(primitive.mesh.clone());
            mesh_render.insert(Transform::from_t_matrix(mesh_mat.clone()));
            
            let shadow = Shadow {cast_shadow:true,receive_shadow:true };
            mesh_render.insert(shadow);
            if let Some(mat) = primitive.material.as_ref().and_then(|v| mat_fn(&v)) {
                mesh_render.insert(mat);
            }
            mesh_list.push(mesh_render.id());
        }
    }
    let mut root = commands.spawn_empty();
    root.insert(Transform::default());
    root.add_children(&mesh_list);
    root.id()
}

