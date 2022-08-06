use std::{collections::HashMap, sync::Arc};
use bevy_ecs::prelude::{Res, ResMut, World};
use crossbeam_channel::{Sender, TryRecvError};
use lite_clojure_eval::EvalRT;
use parking_lot::RwLock;
use seija_asset::{ AssetServer, Assets, Handle, LifecycleEvent, RefEvent, HandleId};
use seija_core::{TypeUuid};
use once_cell::sync::Lazy;


use crate::{material::Material, resource::{color_texture, Texture}};
use super::{MaterialDef, read_material_def};

pub(crate) static DEFAULT_TEXTURES:Lazy<HashMap<String,usize>> = Lazy::new(|| {
    let mut m:HashMap<String,usize> = HashMap::new();
    m.insert("white".to_string(), 0);
    m.insert("blue".to_string(), 1);
    m   
});

pub struct MaterialDefInfo {
   pub def:Arc<MaterialDef>,
   pub mat_count:usize
}

pub struct MaterialStorage {
    pub default_textures:Vec<Handle<Texture>>,
    pub mateials:RwLock<Assets<Material>>,
    pub(crate) name_map:RwLock<HashMap<String,MaterialDefInfo>>,
}

impl MaterialStorage {
    pub fn new(ref_sender:Sender<RefEvent>) -> MaterialStorage {
        MaterialStorage {
            mateials:RwLock::new(Assets::new(ref_sender.clone())),
            name_map:RwLock::new(HashMap::default()),
            default_textures:Vec::new()
        }
    }

    pub fn init(&mut self,world:&mut World) {
        let mut textures = world.get_resource_mut::<Assets<Texture>>().unwrap();
        let white = color_texture([255,255,255,255], 32);
        let h_white = textures.add(white);
        self.default_textures.push(h_white);

        let blue = color_texture([0,0,255,255], 32);
        let h_blue = textures.add(blue);
        self.default_textures.push(h_blue);
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

    pub fn load_material_def(&self,source:&str) -> bool {
        let mut vm = EvalRT::new();
        match read_material_def(&mut vm, source,false) {
            Ok(def) => {
                self.add_def(def);
                return true;
            },
            Err(err) => {
                log::error!("{}",err);
                return false;
             }
        }
    }

    pub fn create_material(&self,name:&str) -> Option<Handle<Material>> {
        let mut name_map = self.name_map.write();
        if let Some(info) = name_map.get_mut(name) {
           let mat = Material::from_def(info.def.clone(),&self.default_textures);
           let handle = self.mateials.write().add(mat);
           info.mat_count += 1;
           return Some(handle);
        }
        None
    }

    pub fn create_material_with(&self,name:&str,f:impl Fn(&mut Material)) -> Option<Handle<Material>> {
        let mut name_map = self.name_map.write();
        if let Some(info) = name_map.get_mut(name) {
            let mut mat = Material::from_def(info.def.clone(),&self.default_textures);
            f(&mut mat);
            let handle = self.mateials.write().add(mat);
            info.mat_count += 1;
            return Some(handle);
        }
        None
    }

    pub fn material_mut<F>(&self,id:&HandleId,f:F) -> bool where F:FnOnce(&mut Material)  {
        let mut mats = self.mateials.write();
        if let Some(mat) = mats.get_mut(id) {
            f(mat);
            return true;
        }
        false
    }

    pub fn has_def(&self,name:&str) -> bool {
        let read_map = self.name_map.read();
        read_map.contains_key(name)
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