use bevy_ecs::{prelude::{World, Entity}};
use seija_asset::Handle;
use seija_core::info::EInfo;
use seija_transform::Transform;

use crate::{material::{MaterialStorage, Material}, camera::camera::Camera};

use super::{system::IQuery, view_list::{ViewList, ViewEntity}};

pub struct CameraQuery {
    camera_entity:Entity
}

impl IQuery for CameraQuery {
    fn on_query(&self,world:&mut World,list:&mut ViewList) {
        list.clear();
        
        let mut views = world.query::<(Entity,&Transform,&Handle<Material>,Option<&EInfo>)>();
        if let Some(materials) = world.get_resource::<MaterialStorage>() {
            let mats = materials.mateials.read();
            let camera = world.entity(self.camera_entity).get::<Camera>();
            let camera_t = world.entity(self.camera_entity).get::<Transform>();
            if let (Some(camera),Some(camera_t)) = (camera,camera_t) {
                let camera_position = camera_t.global().position;
                for (entity,t,m,info) in views.iter(world) {
                    if let Some(info) = info {
                        if info.layer & camera.layer  < 1 { continue; }
                    }
                    let position = t.global().position;
                    let dis_order = (camera_position - position).length_squared();
                    let mat = mats.get(&m.id).unwrap();
                    
                    list.add_entity(mat.order, ViewEntity {entity,order:dis_order });
                }
            }
        }
        list.sort();   
    }
}