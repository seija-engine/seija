use std::ops::{Deref, DerefMut};

use smallvec::SmallVec;
use bevy_ecs::{entity::Entity, prelude::Component};

#[derive(Debug, Copy, Clone, Eq, PartialEq,Component)]
pub struct Parent(pub Entity);

impl Deref for Parent {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Parent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq,Component)]
pub struct PreviousParent(pub(crate) Entity);


#[derive(Default, Clone, Debug,Component)]
pub struct Children(pub(crate) SmallVec<[Entity; 8]>);

impl Children {
    pub fn from(entity: &[Entity]) -> Self {
        Self(SmallVec::from_slice(entity))
    }
}

impl Deref for Children {
    type Target = [Entity];
    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}
