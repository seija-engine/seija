use bevy_ecs::prelude::{Entity, Mut, World};
use fnv::FnvHashMap;
use seija_asset::Handle;
use crate::material::{MaterialDef, storage::MaterialDefInfo};
use wgpu::{Buffer, BufferUsage, CommandEncoder, Device};
use super::{Material, MaterialStorage};


#[derive(Default)]
pub struct MaterialSystem {
    buffers:FnvHashMap<String,BufferInfo>,
}

impl MaterialSystem {
    pub fn update(&mut self,world:&mut World,device:&Device,commands: &mut CommandEncoder) {
        world.resource_scope(|w,storage:Mut<MaterialStorage>| {
            self._update(w, device, storage,commands);
        });
    }
    
    fn _update(&mut self,world:&mut World,device:&Device,storage:Mut<MaterialStorage>,commands: &mut CommandEncoder) {
        {
            let rm_list:Vec<Entity> = world.removed::<Handle<Material>>().collect();
            let name_map_ref = storage.name_map.read();
            for (def_name,mat_def) in  name_map_ref.iter() {
                let buffer = if let Some(buffer) = self.buffers.get_mut(def_name) {
                    buffer.update_size(mat_def.mat_count, device);
                    buffer
                } else {
                    let new_buffer = BufferInfo::new(mat_def,device);
                    self.buffers.insert(def_name.clone(), new_buffer);
                    self.buffers.get_mut(def_name).unwrap()
                };

                for rm_entity in rm_list.iter() {
                    buffer.rm_mat(rm_entity.id())
                }
            }
        }
        
        
        let mut query = world.query::<(Entity,&Handle<Material>)>();
        let mut mats = storage.mateials.write();
        for (e,mat_handle) in query.iter(world) {
            let mat_ref = mats.get_mut(&mat_handle.id).unwrap();
            mat_ref.check_create(device);
            if mat_ref.props.is_dirty() {
                let buffer_info = self.buffers.get_mut(&mat_ref.def.name).unwrap();  
                buffer_info.update(mat_ref, &e,device,commands);
                mat_ref.props.clear_dirty();
            }   
        }

        
       
       
    }   
}



pub struct BufferInfo {
    item_size:usize,
    cur_count:usize,
    buffer:Option<Buffer>,
    len:usize,
    indices:FnvHashMap<u32,usize>,
    free_indices:Vec<usize>
}

impl BufferInfo {
    pub fn new(def_info:&MaterialDefInfo,device:&Device) -> BufferInfo {
        let mut info = BufferInfo {
            item_size:def_info.def.prop_def.size(), 
            cur_count:0, 
            buffer:None,
            indices:FnvHashMap::default(),
            free_indices:Vec::new(),
            len:0
         };
        
        info.update_size(def_info.mat_count,device);
        info
    }

    pub fn rm_mat(&mut self,eid:u32) {
       if let Some(rm_idx) = self.indices.remove(&eid) {
           self.free_indices.push(rm_idx);
       }
    }

    pub fn update_size(&mut self,new_count:usize,device:&Device) {
        if self.cur_count > new_count || new_count == 0 {
            return;
        }
        self.cur_count = (((new_count as u32) + 3u32) & !3u32) as usize;
        let alloc_size = self.cur_count * self.item_size;
        
        let buffer = device.create_buffer(&wgpu::BufferDescriptor { 
            label:None,
            size:alloc_size as u64,
            usage:BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
            mapped_at_creation:false 
        });
        self.buffer = Some(buffer);
    }

    pub fn update(&mut self,mat:&Material,e:&Entity,device:&Device,commands:&mut CommandEncoder) {
        let idx = self.get_or_insert_idx(e.id());
        
        let buffer_mut = self.buffer.as_mut().unwrap();
        let start = idx * self.item_size;
        let end = start + self.item_size;
        {
            let buffer_slice = buffer_mut.slice(..);
            let data = buffer_slice.map_async(wgpu::MapMode::Write);
            device.poll(wgpu::Maintain::Wait);
            if futures_lite::future::block_on(data).is_err() {
                panic!("Failed to map buffer to host.");
            }
            let mut data = buffer_slice.get_mapped_range_mut();
           
            data[start..end].copy_from_slice(mat.props.get_buffer());
        };
        buffer_mut.unmap();

        commands.copy_buffer_to_buffer(buffer_mut, start as u64,
                            mat.buffer.as_ref().unwrap(), 0, self.item_size as u64);

    }

    fn get_or_insert_idx(&mut self,eid:u32) -> usize {
        if let Some(idx) = self.indices.get(&eid) {
            *idx
        } else if self.free_indices.len() > 0 {
            let free_idx = self.free_indices.pop().unwrap();
            self.indices.insert(eid, free_idx);
            free_idx
        } 
        else  {
            let v = self.len;
            self.len += 1;
            self.indices.insert(eid, v);
            v
        }
    }
}

#[test]
fn aaa() {
   
    let n = 8;
    let v:u32 = !3;
    let v2 = (n + 3) & v;
    dbg!(v2);
}