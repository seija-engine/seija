use std::collections::HashSet;
use bevy_ecs::{ prelude::{Entity, Bundle, Events}, 
                world::{World, EntityMut}, 
                system::{Commands, EntityCommands, Command}};
use seija_core::info::EStateInfo;
use smallvec::SmallVec;
use crate::hierarchy::{Children, Parent};

pub enum HierarchyEvent {
    New(Entity),
    Delete(Entity,HashSet<Entity>,Option<Entity>),
    SetParent {
      entity:Entity,
      old_parent:Option<Entity>,
      new_parent:Option<Entity>
    },
    ChildMove {
      entity:Entity,
      parent:Entity,
      index:usize
    },
    SetActive(Entity,bool)
}

struct DeleteCommand(Entity);

struct SetParent {
    pub entity:Entity,
    pub parent:Option<Entity>   
}

struct ChildMove {
    pub entity:Entity,
    parent:Entity,
    pub index:usize
}

struct SetActive {
    pub entity:Entity,
    pub active:bool
}

impl Command for ChildMove {
    fn write(self, world: &mut World) {
        _move_child(world, self.entity,self.parent,self.index);
    }
}

impl Command for SetParent {
    fn write(self, world: &mut World) {
        _set_parent(world, self.entity, self.parent);
    }
}

impl Command for DeleteCommand {
    fn write(self, world: &mut World) { _delete_entity(world, self.0); }
}

impl Command for HierarchyEvent {
    fn write(self, world: &mut World) {
        if let Some(mut event) = world.get_resource_mut::<Events<HierarchyEvent>>() {
            event.send(self);
        }
    }
}

impl Command for SetActive {
    fn write(self, world: &mut World) {
        _set_active(world,self.entity,self.active)
    }
}

pub trait WorldEntityEx {
   fn new_empty(&mut self,send_event:bool) -> Entity;
   fn new<B:Bundle>(&mut self,bundle:B,send_event:bool) -> Entity;
   fn delete(&mut self,entity:Entity);
   fn set_parent(&mut self,entity:Entity,parent:Option<Entity>);
   fn move_child(&mut self,entity:Entity,parent:Entity,index:usize);

   fn set_active(&mut self,entity:Entity,active:bool);
}


impl WorldEntityEx for World {
    fn new_empty(&mut self,send_event:bool) -> Entity {
        let new_entity = self.spawn_empty();
        let eid = new_entity.id();
        if send_event { send_add_event(self, eid); }
        eid
    }

    fn new<B:Bundle>(&mut self,bundle:B,send_event:bool) -> Entity {
        let new_entity = self.spawn(bundle);
        let eid = new_entity.id();
        if send_event { send_add_event(self, eid); }
        eid
    }

    fn set_parent(&mut self,entity:Entity,parent:Option<Entity>) {
        _set_parent(self, entity, parent);
    }

    fn move_child(&mut self,child:Entity,parent:Entity,index:usize) {
        _move_child(self,child,parent,index);
    }

    fn set_active(&mut self,entity:Entity,active:bool) {
        _set_active(self,entity,active)
    }

    fn delete(&mut self,entity:Entity) {
        _delete_entity(self,entity);
    }
}

fn _set_parent(world:&mut World,entity:Entity,parent:Option<Entity>) {
    //从旧Parent中删除
    let old_parent = world.entity_mut(entity).get::<Parent>().map(|p|p.0);
    if let Some(old) = old_parent {
        if let Some(mut children) = world.get_mut::<Children>(old) {
            children.0.retain(|c| *c != entity);
        }
    }
    if let Some(new_parent) = parent {
        //更新Parent
        if let Some(mut parent_comp) = world.entity_mut(entity).get_mut::<Parent>() {
            parent_comp.0 = new_parent;
        } else {
            world.entity_mut(entity).insert(Parent(new_parent));
        }
        //插入到新的Parent的Children
        if let Some(mut parent_children) = world.get_mut::<Children>(new_parent) {
          parent_children.0.push(entity);
        } else {
            world.entity_mut(new_parent).insert(Children(SmallVec::from_slice(&[entity])));
        }   
    } else {
        world.entity_mut(entity).remove::<Parent>();
    }
    if let Some(mut event) = world.get_resource_mut::<Events<HierarchyEvent>>() {
        event.send(HierarchyEvent::SetParent { entity, old_parent, new_parent: parent });
    }
}

fn _move_child(world:&mut World,child:Entity,parent:Entity,index:usize) {
    if let Some(mut children) = world.get_mut::<Children>(parent) {
       let find_idx = children.0.iter().enumerate().find(|e| *e.1 == child).map(|v| v.0);
       if let Some(index) = find_idx {
         children.0.remove(index);
         children.0.insert(index, child);
       }
    }
    if let Some(mut event) = world.get_resource_mut::<Events<HierarchyEvent>>() {
        event.send(HierarchyEvent::ChildMove { entity:parent,parent,index});
    }
}

fn _delete_entity(world:&mut World,entity:Entity) {
    let mut remove_sets:HashSet<Entity> = HashSet::default();
    let mut old_parent = None;
    if let Some(parent) = world.get::<Parent>(entity).map(|parent| parent.0) {
        old_parent = Some(parent);
        if let Some(mut children) = world.get_mut::<Children>(parent) {
            children.0.retain(|c| *c != entity);
        }
    }

    _delete_entity_rec(world, entity, &mut remove_sets);
    
    if let Some(mut event) = world.get_resource_mut::<Events<HierarchyEvent>>() {
        event.send(HierarchyEvent::Delete(entity,remove_sets,old_parent));
    }
}

fn _delete_entity_rec(world:&mut World,entity:Entity,sets:&mut HashSet<Entity>) {
    sets.insert(entity);
    if let Some(mut einfo) = world.get_mut::<EStateInfo>(entity) {
        einfo.is_delete = true;
    } else {
        let mut new_info = EStateInfo::default();
        new_info.is_delete = true;
        world.entity_mut(entity).insert(new_info);
    }
    if let Some(mut children) = world.get_mut::<Children>(entity) {
        for e in std::mem::take(&mut children.0) {
            _delete_entity_rec(world, e,sets);
        }
    }
}

fn _set_active(world:&mut World,entity:Entity,active:bool) {
   if let Some(info) = world.get::<EStateInfo>(entity) {
      info.set_active(active)
   } else {
      let state_info = EStateInfo::default();
      state_info.set_active(active);
      world.entity_mut(entity).insert(state_info);
   }
   if let Some(mut event) = world.get_resource_mut::<Events<HierarchyEvent>>() {
     event.send(HierarchyEvent::SetActive(entity,active));
   }
}

pub trait EntityCommandsEx<'w,'s,'a> {
    fn delete(&mut self);
    fn set_parent(&mut self,parent:Option<Entity>) -> &mut Self;
    fn move_child(&mut self,child:Entity,index:usize);
    fn set_active(&mut self,active:bool);
}

impl<'w,'s,'a> EntityCommandsEx<'w,'s,'a> for EntityCommands<'w, 's, 'a> {
   fn delete(&mut self) {
       let entity = self.id();
       self.commands().add(DeleteCommand(entity));
   }

   fn set_parent(&mut self,parent:Option<Entity>) -> &mut Self {
      let entity = self.id();
      self.commands().add(SetParent { entity,parent });   
      self
   }

   fn move_child(&mut self,child:Entity,index:usize) {
     let cur_entity = self.id();
     self.commands().add(ChildMove { entity:child,parent:cur_entity ,index });
   }

   fn set_active(&mut self,active:bool) {
     let cur_entity = self.id();
     self.commands().add(SetActive { entity:cur_entity,active });
   }
}

pub trait EntityMutEx<'w> {
    fn set_parent(&mut self,parent:Option<Entity>) -> &mut Self;
}

impl<'w> EntityMutEx<'w> for EntityMut<'w> {
    fn set_parent(&mut self,parent:Option<Entity>) -> &mut Self {
        let entity = self.id();
        self.world_scope(|w| {
            _set_parent(w, entity, parent);
        });
        self
    }
}

trait CommandEx<'w,'s> {
    fn new_empty<'a>(&'a mut self,send_event:bool)  -> EntityCommands<'w, 's, 'a>;
    fn new<'a,B:Bundle>(&'a mut self,bundle:B,send_event:bool) -> EntityCommands<'w, 's, 'a>;
}

impl<'w,'s> CommandEx<'w,'s> for Commands<'w,'s> {
    fn new_empty<'a>(&'a mut self,send_event:bool) -> EntityCommands<'w, 's, 'a> {
        let mut cmds = self.spawn_empty();
        
        let entity = cmds.id();
        if send_event {
            cmds.commands().add(HierarchyEvent::New(entity));
        }
        cmds
    }

    fn new<'a,B:Bundle>(&'a mut self,bundle:B,send_event:bool) -> EntityCommands<'w, 's, 'a> {
        let mut cmds = self.spawn(bundle);
        let entity = cmds.id();
        if send_event {
            cmds.commands().add(HierarchyEvent::New(entity));
        }
        cmds
    }
}

fn send_add_event(world:&mut World,entity:Entity) {
    if let Some(mut event) = world.get_resource_mut::<Events<HierarchyEvent>>() {
        event.send(HierarchyEvent::New(entity));
    }
}
