use std::{sync::Arc, collections::HashMap};
use seija_render::{UniformInfo, UniformInfoSet, dsl_frp::FRPDSLSystem};
use smol_str::SmolStr;

pub struct RenderInfo {
    ubos:Vec<Arc<UniformInfo>>,
    pub backend2ubo:HashMap<SmolStr,Arc<UniformInfo>>,
    pub system:FRPDSLSystem
}

impl RenderInfo {
    pub fn new() -> Self {
        RenderInfo { ubos:vec![],backend2ubo:HashMap::default(),system:FRPDSLSystem::new() }
    }

    pub fn run(&mut self,path:&str) {
       match std::fs::read_to_string(path) {
          Ok(code) => {
              let mut info_set:UniformInfoSet = UniformInfoSet::default();
              self.system.init(&code, &mut info_set, &vec![]);
              for (_,ubo_info) in info_set.components.drain() {
                  self.add_ubo_info(ubo_info);
              }
              for (_,ubo_info) in info_set.globals.drain() {
                self.add_ubo_info(ubo_info);
            }
          },
          Err(err) => { log::error!("{:?}",err); }
       }
    }

    fn add_ubo_info(&mut self,info:UniformInfo) {
        let arc_info = Arc::new(info);
        self.ubos.push(arc_info.clone());
        for backend in arc_info.backends.iter() {
            self.backend2ubo.insert(backend.into(), arc_info.clone());
        }
    }
}
