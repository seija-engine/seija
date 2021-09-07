use bevy_ecs::prelude::Entity;

//摄像机可视范围内排序过后的渲染物体
pub struct ViewList {
    pub value:Vec<VisibleEntities>
}


pub struct VisibleEntities {
    pub value: Vec<Entity>,
}