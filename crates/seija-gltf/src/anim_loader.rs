use std::collections::{HashSet, HashMap};

use seija_skeleton3d::offine::raw_skeleton::RawSkeleton;

use crate::{ImportData, GltfError};

pub fn load_skeleton(gltf:&ImportData) -> Result<Option<RawSkeleton>,GltfError> {
    let cur_scene = if let Some(scene) = gltf.0.default_scene() {
        scene
    } else { 
        if let Some(fst) = gltf.0.scenes().next() { fst } else { return Ok(None) }
    };
    if cur_scene.nodes().len() == 0 { return Ok(None) }

    let skins = get_skins_for_scene(&cur_scene,&gltf.0);
   
    Ok(None)
}

fn get_skins_for_scene<'a>(scene:&gltf::Scene<'a>,doc:&'a gltf::Document) -> Vec<gltf::Skin<'a>> {
    let mut open:HashMap<usize,gltf::Node> = HashMap::default();
    let mut found:HashMap<usize,gltf::Node> = HashMap::default();
   
    for node in scene.nodes() {
        open.insert(node.index(),node);
    }

    while !open.is_empty() {
        let node = open.values().next().unwrap().clone();
        open.remove(&node.index());
        found.insert(node.index(),node.clone());
        
        for cnode in node.children() {
            open.insert(cnode.index(), cnode);
        }   
    }

    let mut skins:Vec<gltf::Skin<'a>> = vec![];
    for skin in doc.skins() {
        if let Some(fst) = skin.joints().next() {
            if found.contains_key(&fst.index()) {
                skins.push(skin);
            }
        }
    }
    skins
}