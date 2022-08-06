use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::World;
use bevy_ecs::event::{ManualEventReader, Events};
#[allow(dead_code)]
pub enum TagEvent {
    Start(usize),
    End(usize)
}


pub struct RuntimeTags {
    name2usize:HashMap<String,usize>,
    pub tags:Vec<bool>,
    etag_reader:ManualEventReader<TagEvent>,
    pub dirtys:HashSet<usize>
}

impl RuntimeTags {
    pub fn new() -> Self {
        RuntimeTags { 
            name2usize:HashMap::default(),
            tags:vec![],

            etag_reader:Default::default(),
            dirtys:HashSet::default()
        }
    }

    pub fn name_id(&self,name:&str) -> Option<usize> { self.name2usize.get(name).map(|v| *v) }

    pub fn add_tag(&mut self,name:&str,value:bool) {
        self.tags.push(value);
        let index = self.tags.len() - 1;
        self.name2usize.insert(name.to_string(), index);
    }

    pub fn get_tag(&self,name:&str) -> Option<bool> {
        let id = self.name_id(name)?;
        Some(self.tags[id])
    }

    pub fn update(&mut self,world:&mut World) {
        let tag_events = world.get_resource::<Events<TagEvent>>().unwrap();
        for ev in self.etag_reader.iter(tag_events) {
            
            match ev {
                TagEvent::Start(index) => {
                    if self.tags[*index] == false {  self.dirtys.insert(*index);   }
                    self.tags[*index] = true;
                },
                TagEvent::End(index) => {
                    if self.tags[*index] == true {  self.dirtys.insert(*index);   }
                    self.tags[*index] = false;
                }
            }
        }
    }
}