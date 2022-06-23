use std::collections::{HashMap, HashSet};
use super::uniform_info::{UniformType};
use super::UniformInfo;

#[derive(Default,Debug)]
pub struct UniformInfoSet {
    pub components:HashMap<String,UniformInfo>,
    pub globals:HashMap<String,UniformInfo>,
    backend2ubo:HashMap<String,(String,usize)>
}

impl UniformInfoSet {
    pub fn add_info(&mut self,info:UniformInfo) {
       
        for backend_name in info.backends.iter() {
            self.backend2ubo.insert(backend_name.to_string(), (info.name.to_string(),info.index));
        }
       
        match info.typ {
            UniformType::Component => {
                self.components.insert(info.name.to_string(), info);
            },
            UniformType::Global => {
                self.globals.insert(info.name.to_string(), info);
            },
        }
       
    }

    pub fn get_info(&self,name:&str) -> Option<&UniformInfo> {
        if let Some(info) = self.components.get(name) {
            return Some(info)
        }
        if let Some(info) = self.globals.get(name) {
            return Some(info)
        }
        None
    }
    
    

    pub fn get_ubos_by_backends(&self,backends:&Vec<String>) -> Vec<(String,usize)> {
        let mut ubos:HashSet<String> = HashSet::default();
        let mut ubo_names:Vec<(String,usize)> = vec![];
        for backend_name in backends.iter() {
            if let Some((name,index)) = self.backend2ubo.get(backend_name) {
                if !ubos.contains(name) {
                    ubos.insert(name.clone());
                    ubo_names.push((name.to_string(),*index));
                }
            }
        }
        ubo_names.sort_by(|a,b| a.1.cmp(&b.1));
        ubo_names
    }
}