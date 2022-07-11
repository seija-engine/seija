use bevy_ecs::prelude::{Entity, World};

pub struct QuerySystem {
}

pub trait IQuery {
    fn test(entity:Entity,world:&World) -> bool;
    fn cache();
}