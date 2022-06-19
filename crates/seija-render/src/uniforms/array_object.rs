use std::sync::Arc;

use seija_asset::Handle;
use wgpu::CommandEncoder;

use crate::{UniformInfo, 
    memory::{TypedUniformBuffer, align_num_to}, 
    resource::{RenderResources, BufferId, Texture},
    UniformBufferDef, pipeline::render_bindings::BindGroupBuilder};

use super::texture_def::UniformTextureDef;
pub struct ArrayObjectItem {
    index:usize,
    buffer:TypedUniformBuffer,
    textures:Vec<Handle<Texture>>,
    pub bind_group:Option<wgpu::BindGroup>,

    dirty:bool
}

impl ArrayObjectItem {
    pub fn new(index:usize,
               buffer_def:Arc<UniformBufferDef>,
               texture_def:&Vec<UniformTextureDef>,
               def_texture:&Handle<Texture>) -> ArrayObjectItem {
        let mut textures = vec![];
        for _ in texture_def.iter() {
            textures.push(def_texture.clone_weak());
        }
        ArrayObjectItem {
            index,
            buffer:TypedUniformBuffer::from_def(buffer_def),
            textures,
            bind_group:None,
            dirty:true
        }
    }

    fn update_bind_group(&mut self,item_size:u64,bufferid:&BufferId,res:&RenderResources,layout:&wgpu::BindGroupLayout) {
        let mut build_group_builder = BindGroupBuilder::new();
        let start:u64 = self.index as u64 * item_size;
        build_group_builder.add_buffer_addr(*bufferid, start, item_size);

        self.bind_group = Some(build_group_builder.build(layout, &res.device, res));
        self.dirty = false;
    }

   
}

pub struct ArrayObject {
    buffer_def:Arc<UniformBufferDef>,
    texture_def:Arc<Vec<UniformTextureDef>>,
    layout:wgpu::BindGroupLayout,

    infos:fnv::FnvHashMap<u32,ArrayObjectItem>,
    free_items:Vec<ArrayObjectItem>,

    cap:usize,
    len:usize,
    buffer_item_size:u64,
    cache_buffer:Option<BufferId>,
    buffer:Option<BufferId>
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
            buffer:None
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

    pub fn remove_item(&mut self,eid:u32) {
        if let Some(rm_item) = self.infos.remove(&eid) {
            self.free_items.push(rm_item);   
        }
    }

    fn alloc_buffer(&mut self,count:usize,res:&mut RenderResources) {
        if self.cap == 0 { self.cap = 4; }
        while self.cap < count { self.cap *= 2; }

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
    }

    pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
        for object in self.infos.values_mut().chain(self.free_items.iter_mut()) {
            if res.is_textures_ready(&object.textures) { continue; }
            if let Some(bufferid) = self.buffer.as_ref() {
                object.update_bind_group(self.buffer_item_size,bufferid,res,&self.layout);
            }
        }
    }
}