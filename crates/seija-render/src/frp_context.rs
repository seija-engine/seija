use std::collections::HashMap;

use lite_clojure_eval::Variable;
use lite_clojure_frp::{FRPSystem, DynamicID, EventID};
use parking_lot::RwLock;
use smol_str::SmolStr;


//确保RenderFRPSystem只在主线程使用
unsafe impl Send for FRPContext {}
unsafe impl Sync for FRPContext {}

pub struct FRPContext {
  pub inner:RwLock<FRPContextInner>
}

pub struct FRPContextInner {
    pub system:FRPSystem,
    named_events:HashMap<SmolStr,EventID>,
    named_dyns:HashMap<SmolStr,DynamicID>
}

impl FRPContext {
    pub fn new() -> Self {
        FRPContext {
            inner:FRPContextInner {
                system:FRPSystem::new(),
                named_events:HashMap::default(),
                named_dyns:HashMap::default()
            }.into()
        }
    }
}

impl FRPContextInner {
    pub fn new_event(&mut self,name:Option<SmolStr>) -> EventID {
        let event_id = self.system.new_event(None);
        if let Some(name) = name {
            self.named_events.insert(name, event_id);
        }
        event_id
    }

    pub fn new_dynamic(&mut self,name:Option<SmolStr>,value:Variable) -> DynamicID {
        let dyn_id = self.system.new_dynamic(value, self.system.never(), None).unwrap();
        if let Some(name) = name {
            self.named_dyns.insert(name, dyn_id);
        }
        dyn_id
    }
}