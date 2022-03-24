use std::collections::{HashMap, HashSet};
use super::ubo_info::{UBOType};
use super::UBOInfo;

#[derive(Default,Debug)]
pub struct UBOInfoSet {
    pub component_buffers:HashMap<String,UBOInfo>,
    pub global_buffers:HashMap<String,UBOInfo>,
    backend2ubo:HashMap<String,(String,usize)>
}

impl UBOInfoSet {
    pub fn add_info(&mut self,info:UBOInfo) {
        for backend_name in info.backends.iter() {
            self.backend2ubo.insert(backend_name.to_string(), (info.name.to_string(),info.index));
        }
        match info.typ {
            UBOType::ComponentBuffer => {
                self.component_buffers.insert(info.name.to_string(), info);
            },
            UBOType::GlobalBuffer => {
                self.global_buffers.insert(info.name.to_string(), info);
            },
        }
       
    }

    pub fn get_info(&self,name:&str) -> Option<&UBOInfo> {
        if let Some(info) = self.component_buffers.get(name) {
            return Some(info)
        }
        if let Some(info) = self.global_buffers.get(name) {
            return Some(info)
        }
        None
    }
    
    

    pub fn get_ubos_by_backends(&self,backends:&Vec<String>) -> Vec<(String,usize)> {
        let mut ubos:HashSet<String> = HashSet::default();
        let mut ubo_names:Vec<(String,usize)> = vec![];
        for backend_name in backends.iter() {
            if let Some((name,index)) = self.backend2ubo.get(backend_name) {
                if !ubos.contains(backend_name) {
                    ubos.insert(backend_name.clone());
                    ubo_names.push((name.to_string(),*index));
                }
            }
        }
        ubo_names.sort_by(|a,b| a.1.cmp(&b.1));
        ubo_names
    }
}