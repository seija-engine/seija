use std::collections::HashMap;
use bevy_ecs::prelude::{Res};
use crossbeam_channel::Sender;
use parking_lot::RwLock;
use seija_asset::{AssetEvent, Assets, Handle, HandleId, RefEvent};
use seija_core::event::EventWriter;

use crate::material::Material;

use super::MaterialDef;

pub struct MaterialDefCenter {
    pub(crate) assets:RwLock<Assets<MaterialDef>>,
    name_map:RwLock<HashMap<String,Handle<MaterialDef>>> 
}

impl MaterialDefCenter {
    pub fn new(ref_sender:Sender<RefEvent>) -> MaterialDefCenter {
        MaterialDefCenter {
            assets:RwLock::new(Assets::new(ref_sender)),
            name_map:RwLock::new(HashMap::default())
        }
    }
}



impl MaterialDefCenter {
    pub fn add(&self,mat_def:MaterialDef) -> Handle<MaterialDef> {
        {
            let read_map = self.name_map.read();
            if let Some(id) = read_map.get(&mat_def.name) {
                return id.clone_weak()
            }
        }
        let mut asset_write = self.assets.write();
        let handle:Handle<MaterialDef> = Handle::weak(HandleId::random::<MaterialDef>());
        let clone_name = mat_def.name.clone();
        asset_write.set_untracked(handle.id, mat_def);
        let mut name_map_write = self.name_map.write();
        name_map_write.insert(clone_name, handle.clone_weak());
        handle
    }

    pub fn create_material(&self,name:&str) -> Material {
        
        todo!()
    }
}

pub fn material_center_event(center:Res<MaterialDefCenter>,mut events: EventWriter<AssetEvent<MaterialDef>>,) {
    let mut read_asset = center.assets.write();
    events.send_batch(read_asset.events.drain())
}