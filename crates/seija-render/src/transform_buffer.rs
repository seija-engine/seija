use std::sync::Arc;

use bevy_ecs::prelude::{Changed, Entity, World};
use seija_asset::Handle;
use seija_core::bytes::AsBytes;
use seija_transform::Transform;
use wgpu::{CommandEncoder, Device};

use crate::{material::Material, pipeline::render_bindings::{RenderBindGroup, RenderBindGroupLayout}, resource::{BufferId, Mesh, RenderResourceId, RenderResources}};

const MIN_BUFFER_SIZE:usize = 4;

pub struct TransItem {
    index:usize,
    pub bind_group:RenderBindGroup
}

pub struct TransformBuffer {
    buffer_cap:usize,
    len:usize,
    stage_buffer:Option<BufferId>,
    uniform_buffer:Option<BufferId>,
    infos:fnv::FnvHashMap<u32,TransItem>,
    pub trans_layout:Arc<RenderBindGroupLayout> 
}

impl TransformBuffer {
    pub fn new(device:&Device) -> TransformBuffer {
        let mut trans_layout = RenderBindGroupLayout::default();
        trans_layout.add_layout(wgpu::BindGroupLayoutEntry {
            binding:0,
            visibility:wgpu::ShaderStage::VERTEX,
            ty:wgpu::BindingType::Buffer {
                ty:wgpu::BufferBindingType::Uniform,
                has_dynamic_offset:false,
                min_binding_size:None
            },
            count:None
        });
        trans_layout.build(device);
        TransformBuffer {
            buffer_cap : 0,
            len:0,
            stage_buffer:None,
            uniform_buffer:None,
            infos:fnv::FnvHashMap::default(),
            trans_layout:Arc::new(trans_layout)
        }
    }

    pub fn get_info(&self,eid:&u32) -> Option<&TransItem>  {  self.infos.get(eid) }

    pub fn update(&mut self,world:&mut World,device:&Device,resources:&mut RenderResources,command:&mut CommandEncoder) {
        let mut query = world.query::<(Entity,&Transform,&Handle<Mesh>,&Handle<Material>)>();
        let mut all_count = 0;
        for _ in query.iter(world) {
            all_count += 1;
        }

        if self.buffer_cap < all_count {
            self.alloc_buffer(all_count,resources);
        }


        let mut has_change = false;
        let mut changed_query = world.query_filtered::<Entity,Changed<Transform>>();
        resources.map_buffer(self.stage_buffer.as_ref().unwrap(), wgpu::MapMode::Write);
        for (e,t,_,_) in query.iter(world) {
            if !unsafe { changed_query.get_unchecked(world, e).is_ok() } { continue; }
            has_change = true;
            self.update_item(e.id(),device,resources);
            self.update_buffer(e.id(),t,resources);
        }
        resources.unmap_buffer(self.stage_buffer.as_ref().unwrap());

        if has_change {
            resources.copy_buffer_to_buffer(command,
                                self.stage_buffer.as_ref().unwrap(), 0, 
                            self.uniform_buffer.as_ref().unwrap(), 0, self.buffer_cap as u64 * wgpu::BIND_BUFFER_ALIGNMENT);
        }
    }

    fn update_buffer(&mut self,eid:u32,t:&Transform,resources:&mut RenderResources) {
        let stage_buffer_id = self.stage_buffer.as_ref().unwrap();
        let start:u64 = self.infos.get(&eid).as_ref().unwrap().index as u64 * wgpu::BIND_BUFFER_ALIGNMENT;
        resources.write_mapped_buffer(stage_buffer_id, start..(start + wgpu::BIND_BUFFER_ALIGNMENT),&mut |bytes,_| {
            bytes[0..crate::MATRIX_SIZE as usize].copy_from_slice(t.global().matrix().to_cols_array_2d().as_bytes());
        });

    }

    fn update_item(&mut self,eid:u32,device:&Device,resources:&RenderResources) -> &mut TransItem {
        if !self.infos.contains_key(&eid) {
            let mut bind_group = RenderBindGroup::from_layout(&self.trans_layout);
            let start:u64 = self.len as u64 * wgpu::BIND_BUFFER_ALIGNMENT;
            let res_id = RenderResourceId::BufferAddr(self.uniform_buffer.clone().unwrap(),start,crate::MATRIX_SIZE);             
            bind_group.values.add(res_id);
            bind_group.build(device, resources);

            let new_item = TransItem { index:self.len,bind_group };
            self.len += 1;
            println!("add {}",eid);
            self.infos.insert(eid, new_item);
        }
        self.infos.get_mut(&eid).unwrap()  
    }

    fn alloc_buffer(&mut self,count:usize,resources:&mut RenderResources) {
        if self.buffer_cap <= 0 {
            self.buffer_cap = MIN_BUFFER_SIZE;
        } else {
            while self.buffer_cap < count {
                self.buffer_cap *= 2;
            }
        }
        let new_stage_buffer = resources.create_buffer(&wgpu::BufferDescriptor {
            label:None,
            size: self.buffer_cap as u64 * wgpu::BIND_BUFFER_ALIGNMENT,
            usage: wgpu::BufferUsage::COPY_SRC | wgpu::BufferUsage::MAP_WRITE,
            mapped_at_creation:false
        });
        self.stage_buffer = Some(new_stage_buffer);

        let new_uniform_buffer = resources.create_buffer(&wgpu::BufferDescriptor {
            label:None,
            size: self.buffer_cap as u64 * wgpu::BIND_BUFFER_ALIGNMENT,
            usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM,
            mapped_at_creation:false
        });
        self.uniform_buffer = Some(new_uniform_buffer);
    }
}

