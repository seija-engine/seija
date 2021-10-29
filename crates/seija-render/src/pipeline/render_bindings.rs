use std::{collections::btree_map::Entry, num::{NonZeroU32, NonZeroU64}, sync::Arc};
use wgpu::{BindGroup, BindGroupEntry, Device};

use crate::resource::{BufferId, RenderResourceId, RenderResources};

#[derive(Default,Debug)]
pub struct RenderBindGroupValues {
    entrys:Vec<RenderResourceId>,
}

impl RenderBindGroupValues {
    pub fn add(&mut self,res_id:RenderResourceId) {
        self.entrys.push(res_id);
    }

    pub fn set(&mut self,idx:usize,res_id:RenderResourceId) {
        self.entrys[idx] = res_id;
    }
}

#[derive(Default,Debug)]
pub struct  RenderBindGroupLayout {
    layout_entrys:Vec<wgpu::BindGroupLayoutEntry>,
    pub layout:Option<wgpu::BindGroupLayout>,
}

impl RenderBindGroupLayout {
    pub fn add_layout(&mut self,layout:wgpu::BindGroupLayoutEntry) {
        self.layout_entrys.push(layout);
    }

    pub fn build(&mut self,device:&Device) {
        let desc = wgpu::BindGroupLayoutDescriptor {
            label:None,
            entries:&self.layout_entrys
        };
        let layout = device.create_bind_group_layout(&desc);
        self.layout = Some(layout)
    }
}

#[derive(Default,Debug)]
pub struct RenderBindGroup {
    pub values:RenderBindGroupValues,
    pub layout:Arc<RenderBindGroupLayout>,
    pub bind_group:Option<wgpu::BindGroup>,
}

impl RenderBindGroup {
    pub fn from_layout(layout:&Arc<RenderBindGroupLayout>) -> RenderBindGroup {
        RenderBindGroup {
            values:RenderBindGroupValues::default(),
            layout:layout.clone(),
            bind_group:None
        }
    }

    pub fn build(&mut self,device:&Device,resources:&RenderResources) {
        let mut entrys:Vec<BindGroupEntry> = Vec::new();
        let mut index:u32 = 0;
        for res_id in self.values.entrys.iter() {
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
                &RenderResourceId::BufferAddr(buffer_id,start,size) => {
                    let buffer = resources.buffers.get(&buffer_id).unwrap();
                    entrys.push(BindGroupEntry {
                        binding:index,
                        resource:wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer,
                            offset:start,
                            size:Some(NonZeroU64::new(size).unwrap())
                        }),
                    });
                },
                &RenderResourceId::MainSwap => {  unimplemented!() }
            }
            index+= 1;
        }

        let group_desc = wgpu::BindGroupDescriptor {
            label:None,
            layout:self.layout.layout.as_ref().unwrap(),
            entries:&entrys,
        };
        let bind_group = device.create_bind_group(&group_desc);
        self.bind_group = Some(bind_group);
    }
}

pub struct RenderBindGroupBuilder {

}