use bevy_ecs::prelude::{ Component};
use smol_str::SmolStr;
use std::sync::atomic::{AtomicBool, Ordering};
#[derive(Component, Debug)]
pub struct EInfo {
    pub name: Option<SmolStr>,
    pub layer: i32,
    pub tag: Option<SmolStr>,
    _is_active_global: AtomicBool,
    _is_active: AtomicBool,
    _is_delete: AtomicBool,
}

impl EInfo {
    pub fn is_active(&self) -> bool {
        self._is_active.load(Ordering::SeqCst) && !self.is_delete()
    }

    pub fn is_active_global(&self) -> bool {
        self._is_active_global.load(Ordering::SeqCst) && !self.is_delete()
    }

    pub fn is_delete(&self) -> bool {
        self._is_delete.load(Ordering::SeqCst)
    }

    pub fn set_active(&self, active: bool) {
        self._is_active.store(active, Ordering::SeqCst);
    }

    pub fn set_active_global(&self, active: bool) {
      self._is_active_global.store(active, Ordering::SeqCst);
  }

    pub fn delete(&self) {
        self._is_delete.store(true, Ordering::SeqCst);
    }
}

impl Default for EInfo {
    fn default() -> Self {
        Self {
            name: None,
            layer: 1,
            tag: None,
            _is_active_global:AtomicBool::new(true),
            _is_delete: AtomicBool::new(false),
            _is_active: AtomicBool::new(true),
        }
    }
}
