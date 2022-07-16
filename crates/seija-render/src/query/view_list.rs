use std::cmp::Ordering;
use bevy_ecs::prelude::{Entity};
use crate::material::{RenderOrder};


//摄像机可视范围内排序过后的渲染物体
#[derive(Debug)]
pub struct ViewList {
    pub values:Vec<ViewEntities>
}

impl Default for ViewList {
    fn default() -> Self {
        let mut values =  Vec::new();
        values.resize(RenderOrder::MAX.into(), ViewEntities::default());
        
        Self { values }
    }
}

impl ViewList {
    pub fn clear(&mut self) {
        for v in self.values.iter_mut() {
            v.value.clear();
        }
    }

    pub fn add_entity(&mut self,order:RenderOrder,view_entity:ViewEntity) {
        let idx:usize = order.into();
        self.values[idx].value.push(view_entity);
    }

    pub fn iter(&self)  -> impl Iterator<Item = &Entity> {
        self.values.iter().map(|v| v.value.iter()).flatten().map(|v| &v.entity)
    }

    pub fn sort(&mut self) {
        let idx:usize = RenderOrder::Transparent.into();
        let transparent = &mut self.values[idx];
        transparent.value.sort_by(|a,b| {
            a.order.partial_cmp(&b.order).unwrap_or_else(|| {
                if a.order.is_nan() && !b.order.is_nan() {
                    Ordering::Less
                } else if ! a.order.is_nan() && b.order.is_nan() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }).reverse()
        });
    }
}

#[derive(Default,Clone,Debug)]
pub struct ViewEntities {
    pub value: Vec<ViewEntity>,
}
#[derive(Clone,Debug)]
pub struct ViewEntity {
    pub entity:Entity,
    pub order:f32
}

impl ViewEntity {
    pub fn new(entity:Entity,order:f32) -> ViewEntity {
        ViewEntity {entity,order}
    }
}