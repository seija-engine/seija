use bevy_ecs::{prelude::{Entity, Bundle}, world::World};
#[derive(Debug,Clone)]
pub enum HierarchyEvent {
    ParentChange {
        entity:Entity,
        old_parent:Option<Entity>,
        new_parent:Option<Entity>
    },
    Remove {
        entity:Entity,
        parent:Option<Entity>
    }
}
