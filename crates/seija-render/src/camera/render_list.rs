use bevy_ecs::prelude::Entity;

pub struct RenderList {
    pub value:Vec<VisibleEntities>
}


pub struct VisibleEntities {
    pub value: Vec<Entity>,
}