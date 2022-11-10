use std::collections::HashMap;

use seija_app::ecs::prelude::Entity;
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
    global_events:HashMap<SmolStr,EventID>,
    global_dyns:HashMap<SmolStr,DynamicID>,
    camera_events:HashMap<(Entity,SmolStr),EventID>,
    dynamic_events:HashMap<(Entity,SmolStr),EventID>,
}

impl FRPContext {
    pub fn new() -> Self {
        FRPContext {
            inner:FRPContextInner {
                system:FRPSystem::new(),
                global_events:HashMap::default(),
                global_dyns:HashMap::default(),
                camera_events:HashMap::default(),
                dynamic_events:HashMap::default()
            }.into()
        }
    }
}

impl FRPContextInner {
    pub fn new_event(&mut self,name:Option<SmolStr>) -> EventID {
        let event_id = self.system.new_event(None);
        if let Some(name) = name {
            self.global_events.insert(name, event_id);
        }
        event_id
    }

    pub fn new_dynamic(&mut self,name:Option<SmolStr>,value:Variable) -> DynamicID {
        let dyn_id = self.system.new_dynamic(value, self.system.never(), None).unwrap();
        if let Some(name) = name {
            self.global_dyns.insert(name, dyn_id);
        }
        dyn_id
    }

    pub fn new_camera_event(&mut self,entity:Entity,name:SmolStr) -> EventID {
        let event_id = self.system.new_event(None);
        self.camera_events.insert((entity,name), event_id);
        event_id
    }

    pub fn new_camera_dynamic(&mut self,entity:Entity,name:SmolStr,value:Variable) -> DynamicID {
        let dyn_id = self.system.new_dynamic(value,self.system.never(),None).unwrap();
        self.dynamic_events.insert((entity,name), dyn_id);
        dyn_id
    }
}