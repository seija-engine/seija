use std::collections::HashSet;

use bevy_ecs::{prelude::{Entity, Bundle}, world::{World, EntityMut}, query::Without};
use seija_core::info::EInfo;
/*
startup

preupdate
update
postupdate
preUI
ui
postUI
assetEvents
PreReder
Render
PostRender
Last
*/

pub enum HierarchyEvent {
    New(Entity),
    Delete(Entity),
    RemoveChild {
      parent:Entity,
      entity:Entity,
    },
    AddChild {
        parent:Entity,
        entity:Entity,
    }
}

trait WorldEntityOps {
   fn new_empty(&mut self) -> Entity;
   fn new<B:Bundle>(&mut self) -> Entity;
}

trait EntityHierarchyOps {
    fn remove(self);
    fn add_child(&mut self,entity:Entity,idx:usize);
    fn set_child_index(&mut self,entity:Entity);
}

impl WorldEntityOps for World {
    fn new_empty(&mut self) -> Entity {
        todo!()
    }

    fn new<B:Bundle>(&mut self) -> Entity {
        todo!()
    }
}

fn test(world:&mut World) {
   let mut lst = world.query_filtered::<Entity,Without<EInfo>>();
   let c = lst.iter(world);
}

/*
1. 创建Entity
2. 添加子元素到指定位置
3.删除Entity
*/