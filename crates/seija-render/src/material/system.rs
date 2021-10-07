use bevy_ecs::prelude::{Entity, Mut, World};
use seija_asset::Handle;
use wgpu::{Buffer, Device};

use super::{Material, MaterialStorage};


#[derive(Default)]
pub struct MaterialSystem {
    staging_buffer:Option<Buffer>
}

impl MaterialSystem {
    pub fn update(&mut self,world:&mut World,device:&Device) {
        world.resource_scope(|w,storage:Mut<MaterialStorage>| {
            self._update(w, device, storage);
        });
    }
    
    fn _update(&mut self,world:&mut World,device:&Device,storage:Mut<MaterialStorage>) {
        let mut query = world.query::<(Entity,&Handle<Material>)>();
        let name_map_ref = storage.name_map.read();
        for info in name_map_ref.values() {
            println!("name:{} count:{}",info.def.name,info.mat_count);
        }
    }
       
            
    
}