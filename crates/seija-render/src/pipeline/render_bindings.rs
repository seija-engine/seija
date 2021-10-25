use std::{collections::btree_map::Entry, num::NonZeroU32};
use wgpu::{BindGroupEntry, Device};

use crate::resource::{BufferId, RenderResourceId, RenderResources};

pub struct RenderBinding {
    res_id:RenderResourceId
}

#[derive(Default)]
pub struct RenderBindings {
    entrys:Vec<RenderResourceId>,
    layout_entrys:Vec<wgpu::BindGroupLayoutEntry>,
    
    layout:Option<wgpu::BindGroupLayout>,
    bind_group:Option<wgpu::BindGroup>,
}

impl RenderBindings {
    pub fn add(&mut self,stage:wgpu::ShaderStage,ty:wgpu::BindingType,count:Option<NonZeroU32>,res_id:RenderResourceId) {
        let layout_entry = wgpu::BindGroupLayoutEntry {
            binding:self.layout_entrys.len() as u32,
            visibility:stage,
            ty,
            count
        };
        
        self.layout_entrys.push(layout_entry);
        self.entrys.push(res_id);
    }

    pub fn add_uniform(&mut self,stage:wgpu::ShaderStage,buffer_id:BufferId) {
        self.add(stage, wgpu::BindingType::Buffer {
            ty:wgpu::BufferBindingType::Uniform,
            has_dynamic_offset:false,
            min_binding_size:None

        }, None, RenderResourceId::Buffer(buffer_id));
    }

    pub fn build(&mut self,device:&Device,resources:&RenderResources) {
        let desc = wgpu::BindGroupLayoutDescriptor {
            label:None,
            entries:&self.layout_entrys
        };
        let layout = device.create_bind_group_layout(&desc);

        let mut entrys:Vec<BindGroupEntry> = Vec::new();
        let mut index:u32 = 0;
        for res_id in self.entrys.iter() {
            match res_id {
                &RenderResourceId::Buffer(_) => {
                    let buffer = resources.get_buffer(res_id).unwrap();
                    entrys.push(BindGroupEntry {
                        binding:index,
                        resource:wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer,
                            offset:0,
                            size:None
                        }),
                    });
                }
                &RenderResourceId::MainSwap => {  unimplemented!() }
            }
            index+= 1;
        }

        let group_desc = wgpu::BindGroupDescriptor {
            label:None,
            layout:&layout,
            entries:&entrys,
        };
        let bind_group = device.create_bind_group(&group_desc);

        self.layout = Some(layout);
        self.bind_group = Some(bind_group);

    }
}