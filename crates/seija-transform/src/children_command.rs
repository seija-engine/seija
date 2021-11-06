use bevy_ecs::{prelude::{Entity, World}, system::{Command, EntityCommands}};

use smallvec::{SmallVec};

use crate::hierarchy::{Children, Parent};

#[derive(Debug)]
pub struct PushChildren {
    parent: Entity,
    children: SmallVec<[Entity; 8]>,
}

impl Command for PushChildren {
    fn write(self: Box<Self>, world: &mut World) {
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

impl<'a, 'b> BuildChildren for EntityCommands<'a, 'b> {
    fn add_children(&mut self, children: &[Entity]) -> &mut Self {
        let parent = self.id();
        self.commands().add(PushChildren {
            children: SmallVec::from(children),
            parent,
        });
        self
    }
}