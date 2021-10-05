use bevy_ecs::prelude::{Entity, World};
use wgpu::{Buffer, Device};

use super::Material;


#[derive(Default)]
pub struct MaterialSystem {
    staging_buffer:Option<Buffer>
}

impl MaterialSystem {
    pub fn update(&mut self,world:&mut World,device:&Device) {
        let mut query = world.query::<(Entity,&mut Material)>();
        for (_,mut material) in query.iter_mut(world) {
            material.check_create(device);
        }
    }
}