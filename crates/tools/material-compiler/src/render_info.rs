use std::{sync::Arc, collections::HashMap};
use seija_render::{UBOInfo, RenderScriptContext, UBOInfoSet, RenderGraphContext};

pub struct RenderInfo {
    ubos:Vec<Arc<UBOInfo>>,
    pub backend2ubo:HashMap<String,Arc<UBOInfo>>,
}

impl RenderInfo {
    pub fn new() -> Self {
        RenderInfo { ubos:vec![],backend2ubo:HashMap::default() }
    }

    pub fn run(&mut self,path:&str) {
       let mut rsc = RenderScriptContext::new();
       match std::fs::read_to_string(path) {
          Ok(code) => {
              let mut info_set:UBOInfoSet = UBOInfoSet::default();
              let mut graph_ctx = RenderGraphContext::default();
              rsc.run(code.as_str(), &mut info_set, &mut graph_ctx,false);
              for (_,ubo_info) in info_set.component_buffers.drain() {
                  self.add_ubo_info(ubo_info);
              }
              for (_,ubo_info) in info_set.global_buffers.drain() {
                self.add_ubo_info(ubo_info);
            }
          },
          Err(err) => { log::error!("{:?}",err); }
       }
    }

    fn add_ubo_info(&mut self,info:UBOInfo) {
        let arc_info = Arc::new(info);
        self.ubos.push(arc_info.clone());
        for backend in arc_info.backends.iter() {
            self.backend2ubo.insert(backend.clone(), arc_info.clone());
        }
    }
}
