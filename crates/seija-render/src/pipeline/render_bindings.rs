use std::{ num::{NonZeroU64, NonZeroU32}};
use seija_asset::Handle;
use wgpu::{BindGroupEntry, Device, ShaderStages, TextureView};

use crate::resource::{BufferId, RenderResourceId, RenderResources, Texture, TextureId,SamplerId};

#[derive(Debug,Default)]
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

    pub fn add_sampler(&mut self,filtering:bool) {
        let st = if filtering {wgpu::SamplerBindingType::Filtering } else { wgpu::SamplerBindingType::NonFiltering };
        let entry = wgpu::BindGroupLayoutEntry {
            binding:self.layout_entrys.len() as u32,
            visibility:ShaderStages::VERTEX_FRAGMENT,
            ty:wgpu::BindingType::Sampler(st),
            count:None
        };
        self.layout_entrys.push(entry);
    }


    pub fn add_texture(&mut self,is_cube_map:bool,sample_type:Option<wgpu::TextureSampleType>) {
        let texture_entry = wgpu::BindGroupLayoutEntry {
            binding:self.layout_entrys.len() as u32,
            visibility:ShaderStages::VERTEX_FRAGMENT,
            ty:wgpu::BindingType::Texture {
                sample_type: sample_type.unwrap_or(wgpu::TextureSampleType::Float { filterable: true }),
                view_dimension:if is_cube_map {wgpu::TextureViewDimension::Cube } else { wgpu::TextureViewDimension::D2 },
                multisampled:false
            },
            count:None
        };
        self.layout_entrys.push(texture_entry);
    }

    pub fn add_texture_array(&mut self,count:u32,visibility:wgpu::ShaderStages,sample_type:wgpu::TextureSampleType) {
        let entry = wgpu::BindGroupLayoutEntry {
            binding:self.layout_entrys.len() as u32,
            visibility,
            ty:wgpu::BindingType::Texture { sample_type, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
            count:Some(NonZeroU32::new(count).unwrap())
        };
       
        self.layout_entrys.push(entry);
    }

    pub fn add_uniform(&mut self,stage:wgpu::ShaderStages) {
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
    TextureArray(Vec<TextureId>,SamplerId),
    ResId(RenderResourceId)
}

pub struct BindGroupBuilder {
    entrys:Vec<BindGroupItem>,
    
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

    pub fn is_empty(&self) -> bool {
        self.entrys.is_empty()
    }

    pub fn add_texture_array(&mut self,textures:Vec<TextureId>,sample_id:SamplerId) {
        self.entrys.push(BindGroupItem::TextureArray(textures,sample_id));
    }

    pub fn add_buffer_addr(&mut self,buffer_id:BufferId,start:u64,count:u64) {
        self.entrys.push(BindGroupItem::ResId(RenderResourceId::BufferAddr(buffer_id,start,count)));
    }

    pub fn build(&self,layout:&wgpu::BindGroupLayout,device:&Device,resources:&RenderResources) -> wgpu::BindGroup {
        let mut entrys:Vec<BindGroupEntry> = Vec::new();
        let mut view_list:Vec<Vec<&TextureView>> = vec![];
        let mut index:u32 = 0;
        for item in self.entrys.iter() {
            match item {
                BindGroupItem::TextureArray(textures,_) => {
                    let mut new_lst = vec![];
                    for texture_id in textures.iter() {
                        new_lst.push(resources.get_texture_view(texture_id).unwrap());
                    }
                    view_list.push(new_lst);
                },
                _ => {}
            }
        }

        let mut view_index = 0;
        for item in self.entrys.iter() {
            match item {
                BindGroupItem::ResId(RenderResourceId::Buffer(buffer_id)) => {
                    let buffer = resources.get_buffer(&buffer_id).unwrap();
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
                    let buffer = resources.get_buffer(&buffer_id).unwrap();
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
                BindGroupItem::TextureArray(_,sample_id) => {
                    let entry = BindGroupEntry { binding:index,resource:wgpu::BindingResource::TextureViewArray(view_list[view_index].as_slice()) };
                    entrys.push(entry);
                    index += 1;
                    let sampler = resources.get_sampler(&sample_id).unwrap();
                    let entry = BindGroupEntry { binding:index, resource:wgpu::BindingResource::Sampler(sampler) };
                    entrys.push(entry);
                    index += 1;
                    view_index += 1;
                },
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
