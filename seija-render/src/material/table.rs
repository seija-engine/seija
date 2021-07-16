use std::collections::HashMap;

use bevy_asset::{Handle,AssetServer};
use crate::material::material::MaterialDesc;

#[derive(Default)]
pub struct MaterialDescTable {
    descs:HashMap<String,Handle<MaterialDesc>>
}

impl MaterialDescTable {
    pub fn push(&mut self,path:&str,value:Handle<MaterialDesc>) {
        if !self.descs.contains_key(path) {
            self.descs.insert(path.to_string(), value);
        }
    }

    pub fn get(&mut self,path:&str) -> Option<Handle<MaterialDesc>> {
        self.descs.get(path).map(|v| v.clone())
    }

    pub fn load_get(&mut self,path:&str,assets:&AssetServer) -> Handle<MaterialDesc> {
        if let Some(h) = self.get(path) {
            return h;
        }
        let handle = assets.load(path);
        self.descs.insert(path.to_string(), handle.clone());
        handle
    }

    

}