use bevy_ecs::prelude::Entity;
#[derive(Debug,Clone)]
pub enum HierarchyEvent {
    ParentChange {
        entity:Entity,
        old_parent:Option<Entity>,
        new_parent:Option<Entity>
    }
}