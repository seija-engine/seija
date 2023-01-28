use std::collections::HashMap;

use bevy_ecs::system::Resource;
use parking_lot::RwLock;
use super::view_list::ViewList;

pub struct ViewQuery {
    pub typ:u32,
    pub key:IdOrName,
    pub list:RwLock<ViewList>
}

#[derive(PartialEq,Eq,Hash,Clone,Debug)]
pub enum IdOrName {
    Id(u64),
    Name(String)
}

#[allow(dead_code)]
impl IdOrName {
    pub fn cast_id(&self) -> Option<u64> {
        match self {
            IdOrName::Id(id) => Some(*id),
            IdOrName::Name(_) => None
        }
    }

    pub fn cast_name(&self) -> Option<&String> {
        match self {
            IdOrName::Id(_) => None,
            IdOrName::Name(name) => Some(name)
        }
    }
}

impl ViewQuery {
    pub fn new(typ:u32,key:IdOrName) -> Self {
        ViewQuery { typ, key, list:RwLock::new(ViewList::default()) }
    }
}

#[derive(Default,Resource)]
pub struct QuerySystem {
    key_map:HashMap<IdOrName,usize>,
    pub querys:Vec<ViewQuery>
}

impl QuerySystem {
   pub fn add_query(&mut self,key:IdOrName,typ:u32) -> usize {
      
      let view_query = ViewQuery::new(typ, key.clone());
      self.querys.push(view_query);
      let index = self.querys.len() - 1;
      self.key_map.insert(key, index);
      index
   }

   pub fn rmove_query(&mut self,id:&IdOrName) {
      if let Some(index) = self.key_map.remove(id) {
        self.querys.remove(index);
        for mut_index in self.key_map.values_mut() {
            if *mut_index > index {
                *mut_index = *mut_index - 1;
            }
        }
      }
   }

   pub fn get(&self,id:IdOrName) -> Option<usize> {
      self.key_map.get(&id).map(|v| *v)
   }
}

