use std::sync::atomic::{AtomicBool,Ordering};

use bevy_ecs::prelude::Component;
use smol_str::SmolStr;
#[derive(Component, Debug)]
pub struct EInfo {
    pub name: Option<SmolStr>,
    pub layer: i32,
    pub tag: Option<SmolStr>
}

#[derive(Component, Debug)]
pub struct EStateInfo {
    pub _is_active:AtomicBool,
    pub _is_active_global:bool,
    pub is_delete:bool
}

impl Default for EStateInfo {
    fn default() -> Self {
        EStateInfo { _is_active: AtomicBool::new(true), _is_active_global: true, is_delete: false }
    }
}

impl EStateInfo {
    pub fn is_active_global(&self) -> bool {
        if !self.is_active() { return false }
        self._is_active_global
    }

    pub fn is_active(&self) -> bool {
        self._is_active.load(Ordering::SeqCst) && !self.is_delete
    }
    
    pub fn set_active(&self,active:bool) {
        self._is_active.store(active, Ordering::SeqCst)
    }
}

impl Default for EInfo {
    fn default() -> Self {
        Self {
            name: None,
            layer: 1,
            tag: None
        }
    }
}
