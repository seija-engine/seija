use std::{ num::{NonZeroU64}};
use seija_asset::Handle;
use wgpu::{BindGroupEntry, Device, ShaderStage};

use crate::resource::{BufferId, RenderResourceId, RenderResources, Texture};

#[derive(Debug)]
pub struct BindGroupLayoutBuilder {
    layout_entrys:Vec<wgpu::BindGroupLayoutEntry>
}

impl BindGroupLayoutBuilder {
    pub fn new() -> BindGroupLayoutBuilder {
        BindGroupLayoutBuilder {
            layout_entrys:Vec::new()
        }
    }

    pub fn is_empty(&self) -> bool { self.layout_entrys.is_empty() }

    pub fn add_layout(&mut self,layout:wgpu::BindGroupLayoutEntry) {
        self.layout_entrys.push(layout);
    }

    pub fn add_sampler(&mut self) {
        let entry = wgpu::BindGroupLayoutEntry {
            binding:self.layout_entrys.len() as u32,
            visibility:ShaderStage::FRAGMENT,
            ty:wgpu::BindingType::Sampler {comparison: false, filtering: false },
            count:None
        };
        self.layout_entrys.push(entry);
    }

    pub fn add_texture(&mut self) {
        let texture_entry = wgpu::BindGroupLayoutEntry {
            binding:self.layout_entrys.len() as u32,
            visibility:ShaderStage::FRAGMENT,
            ty:wgpu::BindingType::Texture {
                sample_type:wgpu::TextureSampleType::Float { filterable: true },
                view_dimension:wgpu::TextureViewDimension::D2,
                multisampled:false
            },
            count:None
        };
        self.layout_entrys.push(texture_entry);
    }

    pub fn add_uniform(&mut self,stage:wgpu::ShaderStage) {
        let entry = wgpu::BindGroupLayoutEntry {
            binding:self.layout_entrys.len() as u32,
            visibility:stage,
            ty:wgpu::BindingType::Buffer {
                ty:wgpu::BufferBindingType::Uniform,
                has_dynamic_offset:false,
                min_binding_size:None
            },
            count:None
        };
        self.layout_entrys.push(entry);
    }

    pub fn build(&self,device:&Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:None,
            entries:&self.layout_entrys
        })
    }
}


enum BindGroupItem {
    Texture(Handle<Texture>),
    ResId(RenderResourceId)
}

pub struct BindGroupBuilder {
    entrys:Vec<BindGroupItem>
}

impl BindGroupBuilder {
    pub fn new() -> BindGroupBuilder {
        BindGroupBuilder { entrys: Vec::new() }
    }

    pub fn add_buffer(&mut self,buffer_id:BufferId) {
        self.entrys.push(BindGroupItem::ResId(RenderResourceId::Buffer(buffer_id)));
    }

    pub fn add_texture(&mut self,texture:Handle<Texture>) {
        self.entrys.push(BindGroupItem::Texture(texture));
    }

    pub fn add_buffer_addr(&mut self,buffer_id:BufferId,start:u64,count:u64) {
        self.entrys.push(BindGroupItem::ResId(RenderResourceId::BufferAddr(buffer_id,start,count)));
    }

    pub fn build(&self,layout:&wgpu::BindGroupLayout,device:&Device,resources:&RenderResources) -> wgpu::BindGroup {
        let mut entrys:Vec<BindGroupEntry> = Vec::new();
        let mut index:u32 = 0;
        for item in self.entrys.iter() {
            match item {
                BindGroupItem::ResId(RenderResourceId::Buffer(buffer_id)) => {
                    let buffer = resources.get_buffer(buffer_id).unwrap();
                    entrys.push(BindGroupEntry {
                        binding:index,
                        resource:wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer,
                            offset:0,
                            size:None
                        }),
                    });
                    index += 1;
                },
                BindGroupItem::ResId(RenderResourceId::BufferAddr(buffer_id,start,count)) => {
                    let buffer = resources.get_buffer(buffer_id).unwrap();
                    entrys.push(BindGroupEntry {
                        binding:index,
                        resource:wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer,
                            offset:*start,
                            size:Some(NonZeroU64::new(*count).unwrap())
                        }),
                    });
                    index += 1;
                },
                BindGroupItem::ResId(_) => {},
                BindGroupItem::Texture(texture_handle) => {
                    let res_texture_id = resources.get_render_resource(&texture_handle.id, 0).and_then(|v|v.into_texture_id()).unwrap();
                    let res_sampler_id = resources.get_render_resource(&texture_handle.id, 1).and_then(|v|v.into_sampler_id()).unwrap();
                    let texture_view = resources.get_texture_view(&res_texture_id).unwrap();
                    let sampler = resources.get_sampler(&res_sampler_id).unwrap();

                    let entry = BindGroupEntry { binding:index, resource:wgpu::BindingResource::TextureView(texture_view) };
                    entrys.push(entry);
                    index += 1;
                    let entry = BindGroupEntry { binding:index, resource:wgpu::BindingResource::Sampler(sampler) };
                    entrys.push(entry);
                    index += 1;
                }
            }
        }
        
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:None,
            layout,
            entries:&entrys
        })
    }
}
