use bevy_ecs::{prelude::{Entity, World, Events}, system::{Command, EntityCommands}, world::EntityMut};

use smallvec::{SmallVec};

use crate::{hierarchy::{Children, Parent}, events::HierarchyEvent};

#[derive(Debug)]
pub struct PushChildren {
    pub parent: Entity,
    pub children: SmallVec<[Entity; 8]>,
}

impl PushChildren {
    pub fn new(entity:Entity) -> Self { 
        PushChildren {
            parent:entity,
            children:SmallVec::default()
        } 
    }
}

impl Command for PushChildren {
    fn write(self, world: &mut World) {
        for child in self.children.iter() {
            world.entity_mut(*child).insert(Parent(self.parent));
        }
        if let Some(mut parent_children) = world.get_mut::<Children>(self.parent) {
            parent_children.0.extend(self.children.iter().cloned());
        } else {
            world.entity_mut(self.parent).insert(Children(self.children));
        }
    }
}

pub trait BuildChildren {
    fn add_children(&mut self, children: &[Entity]) -> &mut Self;
}

impl<'a, 'b,'c> BuildChildren for EntityCommands<'a, 'b,'c> {
    fn add_children(&mut self, children: &[Entity]) -> &mut Self {
        let parent = self.id();
        self.commands().add(PushChildren {
            children: SmallVec::from(children),
            parent,
        });
        self
    }
}


#[derive(Debug)]
pub struct DespawnRecursive {
    pub entity: Entity,
}

pub struct SetParent {
    pub entity:Entity,
    pub parent:Option<Entity>   
}

impl Command for DespawnRecursive {
    fn write(self, world: &mut World) {
        despawn_with_children_recursive(world, self.entity);
    }
}

impl Command for SetParent {
    fn write(self, world: &mut World) {
        world.entity_mut(self.entity).set_parent(self.parent);
    }
}


fn despawn_with_children_recursive_inner(world: &mut World, entity: Entity) {
    if let Some(mut children) = world.get_mut::<Children>(entity) {
        for e in std::mem::take(&mut children.0) {
            despawn_with_children_recursive_inner(world, e);
        }
    }

    if !world.despawn(entity) {
        log::error!("Failed to despawn entity {:?}", entity);
    }
}


pub trait IEntityChildren {
    fn despawn_recursive(self);
    fn set_parent(&mut self,parent:Option<Entity>);
}

impl<'a, 'b,'c> IEntityChildren for EntityCommands<'a, 'b,'c> {
    fn despawn_recursive(mut self) {
        let entity = self.id();
        self.commands().add(DespawnRecursive { entity });
    }

    fn set_parent(&mut self,parent:Option<Entity>) {
        let entity = self.id();
        self.commands().add(SetParent {entity,parent });
    }
}

impl<'w> IEntityChildren for EntityMut<'w> {
    fn despawn_recursive(mut self) {
        let entity = self.id();
        let world_mut = unsafe { self.world_mut() };
        despawn_with_children_recursive(world_mut, entity);
        
    }

    fn set_parent(&mut self,parent:Option<Entity>) {
        let cur_entity = self.id();
        self.world_scope(|w| {
            let old_parent = w.entity_mut(cur_entity).get::<Parent>().map(|p|p.0);
            if let Some(old) = old_parent {
                if let Some(mut children) = w.get_mut::<Children>(old) {
                    children.0.retain(|c| *c != cur_entity);
                }
            }
            if let Some(new_parent) = parent {
                w.entity_mut(cur_entity).get_mut::<Parent>().map(|mut p| p.0 = new_parent);
                if let Some(mut parent_children) = w.get_mut::<Children>(new_parent) {
                  parent_children.0.push(cur_entity);
                } else {
                    w.entity_mut(new_parent).insert(Children(SmallVec::from_slice(&[cur_entity])));
                }   
            } else {
                w.entity_mut(cur_entity).remove::<Parent>();
            }
            if old_parent != parent {
                if let Some(mut event) = w.get_resource_mut::<Events<HierarchyEvent>>() {
                    event.send(HierarchyEvent::ParentChange { entity:cur_entity, old_parent, new_parent: parent });
                }
            }
        })
    }
}

pub fn despawn_with_children_recursive(world: &mut World, entity: Entity) {
    if let Some(parent) = world.get::<Parent>(entity).map(|parent| parent.0) {
        if let Some(mut children) = world.get_mut::<Children>(parent) {
            children.0.retain(|c| *c != entity);
        }
    }
    despawn_with_children_recursive_inner(world, entity);
}

