use std::{sync::Arc, collections::HashMap};

use seija_asset::Handle;
use wgpu::CommandEncoder;

use crate::{UniformInfo, 
    memory::{TypedUniformBuffer, align_num_to}, 
    resource::{RenderResources, BufferId, Texture},
    UniformBufferDef, pipeline::render_bindings::BindGroupBuilder};

use super::texture_def::UniformTextureDef;
pub struct ArrayObjectItem {
    index:usize,
    pub buffer:TypedUniformBuffer,
    texture_idxs:HashMap<String,usize>,
    textures:Vec<Handle<Texture>>,
    pub bind_group:Option<wgpu::BindGroup>,

    texture_dirty:bool
}

impl ArrayObjectItem {
    pub fn new(index:usize,
               buffer_def:Arc<UniformBufferDef>,
               texture_def:&Vec<UniformTextureDef>,
               def_texture:&Handle<Texture>) -> ArrayObjectItem {
        let mut textures = vec![];
        let mut texture_idxs = HashMap::default();
        for (idx,def) in texture_def.iter().enumerate() {
            textures.push(def_texture.clone_weak());
            texture_idxs.insert(def.name.clone(), idx);
        }
        ArrayObjectItem {
            index,
            buffer:TypedUniformBuffer::from_def(buffer_def),
            textures,
            bind_group:None,
            texture_dirty:true,
            texture_idxs
        }
    }

    fn update_bind_group(&mut self,item_size:u64,bufferid:&BufferId,res:&RenderResources,layout:&wgpu::BindGroupLayout) {
        let mut build_group_builder = BindGroupBuilder::new();
        let start:u64 = self.index as u64 * item_size;
        build_group_builder.add_buffer_addr(*bufferid, start, item_size);

        self.bind_group = Some(build_group_builder.build(layout, &res.device, res));
       
        self.texture_dirty = false;
    }

    pub fn set_texture(&mut self,name:&str,texture:Handle<Texture>) {
        if let Some(index) = self.texture_idxs.get(name) {
            self.textures[*index] = texture.clone();
            self.texture_dirty = true;
        }
    }

    
   
}

pub struct ArrayObject {
    buffer_def:Arc<UniformBufferDef>,
    texture_def:Arc<Vec<UniformTextureDef>>,
    pub layout:wgpu::BindGroupLayout,

    infos:fnv::FnvHashMap<u32,ArrayObjectItem>,
    free_items:Vec<ArrayObjectItem>,

    cap:usize,
    len:usize,
    buffer_item_size:u64,
    cache_buffer:Option<BufferId>,
    buffer:Option<BufferId>,

    buffer_dirty:bool
}

impl ArrayObject {
    pub fn new(info:&UniformInfo,res:&RenderResources) -> Self {
        let buffer_item_size:u64 = align_num_to(info.props.size() as u64, wgpu::BIND_BUFFER_ALIGNMENT);
        ArrayObject {
            buffer_def:info.props.clone(),
            texture_def:info.textures.clone(),
            layout:info.create_layout(&res.device),
            infos:fnv::FnvHashMap::default(),
            free_items:vec![],
            cap:0,
            len:0,
            buffer_item_size,
            cache_buffer:None,
            buffer:None,
            buffer_dirty:false
        }
    }

    pub fn add_item(&mut self,eid:u32,res:&mut RenderResources) {
        if self.infos.contains_key(&eid) { return; }
        if let Some(free_item) = self.free_items.pop() {
            self.infos.insert(eid, free_item);
            return;
        }

        self.len += 1;
        if self.cap < self.len { self.alloc_buffer(self.len, res); }

        let index = self.len - 1;
        let item = ArrayObjectItem::new(index,
                                                         self.buffer_def.clone(),
                                                         &self.texture_def,
                                             &res.default_textures[0]);

        self.infos.insert(eid, item);
    }

    pub fn get_item_mut(&mut self,eid:u32) -> Option<&mut ArrayObjectItem> {
        self.infos.get_mut(&eid)
    }

    pub fn get_item(&self,eid:u32) -> Option<&ArrayObjectItem> {
        self.infos.get(&eid)
    }

    pub fn remove_item(&mut self,eid:u32) {
        if let Some(rm_item) = self.infos.remove(&eid) {
            self.free_items.push(rm_item);   
        }
    }

    fn alloc_buffer(&mut self,count:usize,res:&mut RenderResources) {
        if self.cap == 0 { self.cap = 4; }
        while self.cap < count { self.cap *= 2; }
        log::info!("array object alloc:{}",self.cap);
        let cache_buffer = res.create_buffer(&wgpu::BufferDescriptor {
            label:None,
            size: self.cap as u64 * self.buffer_item_size as u64,
            usage: wgpu::BufferUsage::COPY_SRC | wgpu::BufferUsage::MAP_WRITE,
            mapped_at_creation:false
        });
        self.cache_buffer = Some(cache_buffer);

        let uniform_buffer = res.create_buffer(&wgpu::BufferDescriptor {
            label:None,
            size: self.cap as u64 * self.buffer_item_size as u64,
            usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM,
            mapped_at_creation:false
        });
        self.buffer = Some(uniform_buffer);
        self.buffer_dirty = true;
        
        for cache_buffer in self.infos.values_mut().chain(self.free_items.iter_mut()) {
            cache_buffer.buffer.set_dirty();
        }
    }

    pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
        let mut is_buffer_changed = false;
        //update bind group
        for object in self.infos.values_mut().chain(self.free_items.iter_mut()) {
            if object.buffer.is_dirty() { is_buffer_changed = true; }
            if self.buffer_dirty {
                if let Some(bufferid) = self.buffer.as_ref() {
                    object.update_bind_group(self.buffer_item_size,bufferid,res,&self.layout);
                }
                is_buffer_changed = true;                
            } else {
                if !res.is_textures_ready(&object.textures) || !object.texture_dirty { continue; }
                if let Some(bufferid) = self.buffer.as_ref() {
                    object.update_bind_group(self.buffer_item_size,bufferid,res,&self.layout);
                }
            }
        }

        if is_buffer_changed {
            self.update_buffer(res, cmd);
        }
        self.buffer_dirty = false;
    }

    pub fn update_buffer(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
        if let Some(cache_id) = self.cache_buffer {
            res.map_buffer(&cache_id, wgpu::MapMode::Write);
            for object in self.infos.values_mut().chain(self.free_items.iter_mut()) {
                if object.buffer.is_dirty() {
                    let start = object.index  as u64 * self.buffer_item_size;
                    let buffer = object.buffer.get_buffer();
                    res.write_mapped_buffer(&cache_id, start..(start + buffer.len() as u64), &mut |bytes,_| {
                        bytes[0..buffer.len()].copy_from_slice(buffer);
                   });
                   object.buffer.clear_dirty();
                }
            }
            res.unmap_buffer(&cache_id);
            res.copy_buffer_to_buffer(cmd,
                            &cache_id, 
                            0, 
                        self.buffer.as_ref().unwrap(),
                        0,
                                    self.cap as u64 * self.buffer_item_size);
        }
    }
}