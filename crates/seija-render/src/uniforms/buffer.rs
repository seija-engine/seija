use wgpu::CommandEncoder;
use crate::{memory::TypedUniformBuffer, resource::{BufferId, RenderResources}, UBOInfo, pipeline::render_bindings::{BindGroupLayoutBuilder, BindGroupBuilder}};


pub struct UBObject {
    pub local:TypedUniformBuffer,
    cache:Option<BufferId>,
    pub buffer:BufferId,
    pub bind_group:wgpu::BindGroup
}

impl UBObject {
    pub fn create(info:&UBOInfo,res:&mut RenderResources,layout:&wgpu::BindGroupLayout) -> Self {
        let local = TypedUniformBuffer::from_def(info.props.clone());
        let buffer = res.create_buffer(&wgpu::BufferDescriptor {
            label:None,
            size:info.props.size() as u64,
            usage:wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM,
            mapped_at_creation:false
        });

        let mut group_builder = BindGroupBuilder::new();
        group_builder.add_buffer(buffer);
        let bind_group = group_builder.build(&layout, &res.device, res);
        UBObject {local, cache:None,buffer,bind_group }
    }

    pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
        if !self.local.is_dirty() { return; }
        let buffer_size = self.local.def.size() as u64;
        let cache_id = match self.cache {
            Some(cache_id) => {
                res.map_buffer(&cache_id, wgpu::MapMode::Write);
                res.write_mapped_buffer(&cache_id, 0.. buffer_size,&mut |bytes,_| {
                    bytes[0..buffer_size as usize].copy_from_slice(self.local.get_buffer());
                });
                res.unmap_buffer(&cache_id);
                cache_id
            },
            None => {
                let cache_id = res.create_buffer(&wgpu::BufferDescriptor {
                    label:None,
                    size:self.local.def.size() as u64,
                    usage:wgpu::BufferUsage::COPY_SRC | wgpu::BufferUsage::MAP_WRITE,
                    mapped_at_creation:false
                });
                self.cache = Some(cache_id);
                cache_id
            }
        };

        res.copy_buffer_to_buffer(cmd,
                                   &cache_id,
                                   0,
                               &self.buffer,
                               0, 
                                           self.local.def.size() as u64);
        self.local.clear_dirty();
    }
}