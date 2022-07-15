use bevy_ecs::{prelude::{World,Entity}};
use parking_lot::RwLock;
use super::view_list::ViewList;

pub struct ViewQuery {
    pub typ:u32,
    pub key:u64,
    list:ViewList
}

impl ViewQuery {
    pub fn new(typ:u32,key:u64) -> Self {
        ViewQuery { typ, key, list:ViewList::default() }
    }
}

#[derive(Default)]
pub struct QuerySystem {
    pub querys:Vec<RwLock<ViewQuery>>
}

impl QuerySystem {
    pub fn remove_by_key(&mut self,key:u64) {
       
    }
}

