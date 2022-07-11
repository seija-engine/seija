use std::marker::PhantomData;

use bevy_ecs::{prelude::{World,Entity}};

use super::view_list::ViewList;


pub trait IQuery {
    fn on_query(&self,world:&mut World,list:&mut ViewList);
}

pub struct QueryContext {
    query:Box<dyn IQuery + Send + Sync>,
    list:ViewList
}

impl QueryContext {
    pub fn update(&mut self,world:&mut World) {
        self.query.on_query(world, &mut self.list);
    }
}

#[derive(Default)]
pub struct QuerySystem {
    querys:Vec<QueryContext>
}


impl QuerySystem {
    pub fn update(&mut self,world:&mut World) {
        for query in self.querys.iter_mut() {
            query.update(world);
        }
    }

    pub fn iter_query(&self,index:usize) -> Option<impl Iterator<Item = &Entity>>   {
        self.querys.get(index).map(|v| v.list.list())
    }
}

