use std::cmp::Ordering;

use bevy_ecs::prelude::{Entity, Query, Res};
use seija_asset::Handle;
use seija_transform::Transform;

use crate::material::{Material, MaterialStorage, RenderOrder};

use super::camera::Camera;

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
    order:f32
}

impl ViewEntity {
    pub fn new(entity:Entity,order:f32) -> ViewEntity {
        ViewEntity {entity,order}
    }
}


pub(crate) fn view_list_system(mut camera_query: Query<(&mut Camera,&Transform)>,
                               view_query:Query<(Entity,&Transform,&Handle<Material>)>,
                               mat_storage:Res<MaterialStorage>) {
   
    for (mut camera,camera_trans) in camera_query.iter_mut() {
        camera.view_list.clear();
        let mats = mat_storage.mateials.read();
        let camera_position = camera_trans.global().position;
       
        for (entity, trans, matid) in view_query.iter() {
           
            let position = trans.global().position;
            let dis_order = (camera_position - position).length_squared();
            let mat = mats.get(&matid.id).unwrap();
            camera.view_list.add_entity(mat.order, ViewEntity {entity,order:dis_order });
        }
        camera.view_list.sort();
    }
}