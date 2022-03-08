use std::collections::HashMap;
use super::ubo_info::{UBOType};
use super::UBOInfo;

#[derive(Default,Debug)]
pub struct UBOInfoSet {
    pub per_cameras:HashMap<String,UBOInfo>,
    pub per_frames:HashMap<String,UBOInfo>,
    pub per_objects:HashMap<String,UBOInfo>
}

impl UBOInfoSet {
    pub fn add_info(&mut self,info:UBOInfo) {
        match info.typ {
            UBOType::PerCamera => {
                self.per_cameras.insert(info.name.to_string(), info);
            },
            UBOType::PerObject => {
                self.per_objects.insert(info.name.to_string(), info);
            },
            UBOType::PerFrame  => {
                self.per_frames.insert(info.name.to_string(), info);
            }
        }
    }

    pub fn get_info(&self,name:&str) -> Option<&UBOInfo> {
        if let Some(info) = self.per_cameras.get(name) {
            return Some(info)
        }
        if let Some(info) = self.per_frames.get(name) {
            return Some(info)
        }
        if let Some(info) = self.per_objects.get(name) {
            return Some(info)
        }
        None
    }
}