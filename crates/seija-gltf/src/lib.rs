
pub mod asset;
pub mod loader;
mod errors;
//mod anim_loader;
use asset::{GltfAsset, GltfNode,GltfMaterial};
use bevy_ecs::prelude::{Commands, Entity};
pub use errors::GltfError;
use glam::Vec3;
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

pub fn create_gltf<T>(pos:Vec3,asset:&GltfAsset,commands:&mut Commands,mat_fn:&T) -> Entity  where T: Fn(&GltfMaterial) -> Option<Handle<Material>> {
    let mut scene_entitys:Vec<Entity> = vec![];
    for scene in asset.scenes.iter() {
        let mut nodes:Vec<Entity> = vec![];
        for node_index in scene.nodes.iter() {
          let node = create_node(&asset.nodes[*node_index], &asset, commands,& mat_fn);
          nodes.push(node);
        }
        let mut scene_ecmd = commands.spawn();
        scene_ecmd.add_children(&nodes);
        let scene_id = scene_ecmd.insert(Transform::default()).id();
        scene_entitys.push(scene_id);
    }

    let mut root_t = Transform::default();
    root_t.local.position = pos;
    commands.spawn().insert(root_t).add_children(&scene_entitys).id()
}

pub fn create_node<T>(cur_node:&GltfNode,asset:&GltfAsset,commands:&mut Commands,mat_fn:&T) -> Entity 
where T: Fn(&GltfMaterial) -> Option<Handle<Material>> {
    let mut child_nodes:Vec<Entity> = vec![];
    for cnode_index in cur_node.children.iter() {
        let child = create_node(&asset.nodes[*cnode_index], asset, commands,mat_fn);
        child_nodes.push(child);
    }

    
    if let Some(mesh_index) = cur_node.mesh {
        let mesh = &asset.meshs[mesh_index];
        for primitive in mesh.primitives.iter() {
            let mut mesh_render = commands.spawn();
            mesh_render.insert(primitive.mesh.clone());
            mesh_render.insert(Transform::default());
            if let Some(mat) = primitive.material.as_ref().and_then(|v| mat_fn(&v)) {
                mesh_render.insert(mat);
            }
            child_nodes.push(mesh_render.id());
        }
    }

    let mut node = commands.spawn();
    node.add_children(&child_nodes);
    node.insert(Transform::from_t_matrix(cur_node.transform.local.clone()));
    node.id()
}