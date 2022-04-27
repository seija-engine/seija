use std::sync::Arc;

use crate::{resource::{BufferId, RenderResources}, memory::{TypedUniformBuffer, UniformBufferDef, align_num_to}, pipeline::render_bindings::BindGroupBuilder};
use fnv::FnvHashMap;
use wgpu::CommandEncoder;
 
pub struct ArrayItem {
    index:usize,
    buffer:TypedUniformBuffer,
    pub bind_group:wgpu::BindGroup
}

pub struct UBOArrayBuffer {
    buffer_def:Arc<UniformBufferDef>,
    item_size:u64,
    cap:usize,
    len:usize,
    cache:Option<BufferId>,
    buffer:Option<BufferId>,
    infos:fnv::FnvHashMap<u32,ArrayItem>,
    free_items:Vec<ArrayItem>,
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
            infos: FnvHashMap::default(),
            free_items:Vec::new()
        }
    }

    pub fn add_item(&mut self,eid:u32,res:&mut RenderResources,layout:&wgpu::BindGroupLayout) {
        if !self.infos.contains_key(&eid) {
            if let Some(free_item) = self.free_items.pop() {
                self.infos.insert(eid, free_item);
                return;
            }
            self.len += 1;
            if self.cap < self.len {
                self.alloc_buffer(self.len,layout, res);
            }

            let index = self.len - 1;
            let mut build_group_builder = BindGroupBuilder::new();
            let start:u64 = index as u64 * self.item_size;
            build_group_builder.add_buffer_addr(self.buffer.unwrap(), start, self.item_size);
            self.infos.insert(eid, ArrayItem {
                index : index,
                buffer: TypedUniformBuffer::from_def(self.buffer_def.clone()),
                bind_group:build_group_builder.build(layout, &res.device, res),
            });
        }
    }

    

    pub fn remove_item(&mut self,eid:u32) {
        if let Some(rm_item) = self.infos.remove(&eid) {
            self.free_items.push(rm_item);   
        }
    }

    pub fn get_item_buffer_mut(&mut self,eid:u32) -> Option<&mut TypedUniformBuffer> {
        self.infos.get_mut(&eid).map(|v| &mut v.buffer)
    }

    pub fn get_item(&self,eid:u32) -> Option<&ArrayItem> {
        self.infos.get(&eid)
    }

    pub fn alloc_buffer(&mut self,count:usize,layout:&wgpu::BindGroupLayout,res:&mut RenderResources) {
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
        self.buffer = Some(uniform_buffer);
        self.re_bind_group(layout,res);
    }

    fn re_bind_group(&mut self,layout:&wgpu::BindGroupLayout,res:&mut RenderResources) {
        for item in self.infos.values_mut() {
            let mut build_group_builder = BindGroupBuilder::new();
            let start:u64 = item.index as u64 * self.item_size;
            build_group_builder.add_buffer_addr(self.buffer.unwrap(), start, self.item_size);
            item.bind_group = build_group_builder.build(layout, &res.device, res);
        }
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