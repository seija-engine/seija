use std::{borrow::BorrowMut, collections::HashMap, sync::Arc};
use bevy_ecs::prelude::{Res, ResMut};
use crossbeam_channel::{Sender, TryRecvError};
use parking_lot::RwLock;
use seija_asset::{AssetEvent, AssetServer, Assets, Handle, HandleId, LifecycleEvent, RefEvent};
use seija_core::{TypeUuid, event::EventWriter};

use crate::material::Material;
use super::MaterialDef;

pub struct MaterialDefInfo {
   pub def:Arc<MaterialDef>,
   pub mat_count:u32
}

pub struct MaterialStorage {
    pub(crate) mateials:RwLock<Assets<Material>>,
    pub(crate) name_map:RwLock<HashMap<String,MaterialDefInfo>>,
}

impl MaterialStorage {
    pub fn new(ref_sender:Sender<RefEvent>) -> MaterialStorage {
        MaterialStorage {
            mateials:RwLock::new(Assets::new(ref_sender.clone())),
            name_map:RwLock::new(HashMap::default())
        }
    }
}

impl MaterialStorage {
    pub fn add_def(&self,mat_def:MaterialDef) {
        if  self.name_map.read().contains_key(&mat_def.name) {
            return;
        }
        
        let clone_name = mat_def.name.clone();
        let mut name_map_write = self.name_map.write();
        name_map_write.insert(clone_name, MaterialDefInfo {
            def:Arc::new(mat_def),
            mat_count:0
        });
    }

    pub fn create_material(&self,name:&str) -> Option<Handle<Material>> {
        let mut name_map = self.name_map.write();
       
        if let Some(info) = name_map.get_mut(name) {
           let mat = Material::from_def(info.def.clone());
           let handle = self.mateials.write().add(mat);
           info.mat_count += 1;
           return Some(handle);
        }
        None
    }
}

pub fn material_storage_event(server:Res<AssetServer>,storage:ResMut<MaterialStorage>) {
    
    let life_events = server.lifecycle_events.read();
    let life_event = life_events.get(&Material::TYPE_UUID).unwrap();
    
    loop {
        match life_event.receiver.try_recv() {
            Ok(LifecycleEvent::Create(_id)) => { },
            Ok(LifecycleEvent::Free(id)) => {
                let mat = storage.mateials.write().remove(id).unwrap();
                let mut name_map = storage.name_map.write();
                if let Some(info) = name_map.get_mut(&mat.def.name) {
                    info.mat_count -= 1;
                }
            },
            Err(TryRecvError::Empty) => {
                break;
            }
            Err(TryRecvError::Disconnected) => panic!("AssetChannel disconnected."),
        }
    }
}