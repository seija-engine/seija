use std::sync::Arc;

use crate::{resource::{BufferId, RenderResources}, memory::{TypedUniformBuffer, UniformBufferDef, align_num_to}};
use fnv::FnvHashMap;
use wgpu::CommandEncoder;

struct ArrayItem {
    index:usize,
    buffer:TypedUniformBuffer
}

pub struct UBOArrayBuffer {
    buffer_def:Arc<UniformBufferDef>,
    item_size:u64,
    cap:usize,
    len:usize,
    cache:Option<BufferId>,
    buffer:Option<BufferId>,
    infos:fnv::FnvHashMap<u32,ArrayItem>,
}

impl UBOArrayBuffer {
    pub fn new(buffer_def:Arc<UniformBufferDef>) -> Self {
        let item_size:u64 = align_num_to(buffer_def.size() as u64, wgpu::BIND_BUFFER_ALIGNMENT);
       
        UBOArrayBuffer { 
            item_size,
            buffer_def,
            cap : 0, 
            len : 0, 
            cache : None, 
            buffer: None, 
            infos: FnvHashMap::default()
        }
    }

    pub fn add_item(&mut self,eid:u32,res:&mut RenderResources) {
        if !self.infos.contains_key(&eid) {
            let array_item = ArrayItem {
                index : self.len,
                buffer: TypedUniformBuffer::from_def(self.buffer_def.clone())
            };
            self.infos.insert(eid, array_item);
            self.len += 1;
            if self.cap < self.len {
                self.alloc_buffer(self.len, res);
            }
        }
    }

    pub fn get_item_buffer_mut(&mut self,eid:u32) -> Option<&mut TypedUniformBuffer> {
        self.infos.get_mut(&eid).map(|v| &mut v.buffer)
    }

    pub fn alloc_buffer(&mut self,count:usize,res:&mut RenderResources) {
        if self.cap == 0 {
            self.cap = 4;
        }
        while self.cap < count {
            self.cap *= 2;
        }

        let cache_buffer = res.create_buffer(&wgpu::BufferDescriptor {
            label:None,
            size: self.cap as u64 * self.item_size as u64,
            usage: wgpu::BufferUsage::COPY_SRC | wgpu::BufferUsage::MAP_WRITE,
            mapped_at_creation:false
        });
        self.cache = Some(cache_buffer);

        let uniform_buffer = res.create_buffer(&wgpu::BufferDescriptor {
            label:None,
            size: self.cap as u64 * self.item_size as u64,
            usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM,
            mapped_at_creation:false
        });
        self.buffer = Some(uniform_buffer)
    }

   

    pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
       let has_change = self.infos.values().any(|v| v.buffer.is_dirty());
       if !has_change || self.cache.is_none() {  return }

       let cache_id = self.cache.as_ref().unwrap();
       res.map_buffer(cache_id, wgpu::MapMode::Write);
       for item in self.infos.values() {
           if item.buffer.is_dirty() {
               let start = item.index  as u64 * self.item_size;
               let buffer = item.buffer.get_buffer();
               res.write_mapped_buffer(cache_id, start..(start + buffer.len() as u64), &mut |bytes,_| {
                    bytes[0..buffer.len()].copy_from_slice(buffer);
               });
           }
       }
       res.unmap_buffer(cache_id);
       self.infos.values_mut().for_each(|v| v.buffer.clear_dirty());
       res.copy_buffer_to_buffer(cmd,&cache_id, 0, self.buffer.as_ref().unwrap(), 0,self.cap as u64 * self.item_size);
       
    }
}