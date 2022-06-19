use seija_asset::Handle;
use wgpu::CommandEncoder;

use crate::{resource::{RenderResources, BufferId, Texture}, UniformInfo, memory::TypedUniformBuffer, pipeline::render_bindings::BindGroupBuilder};

pub struct UniformObject {
    //buffer
    pub local_buffer:TypedUniformBuffer,
    buffer:BufferId,
    cache_buffer:Option<BufferId>,
    //texture
    textures:Vec<Handle<Texture>>,
    texture_dirty:bool,
    pub layout:wgpu::BindGroupLayout,
    pub bind_group:Option<wgpu::BindGroup>
}

impl UniformObject {
    pub fn new(res:&mut RenderResources,info:&UniformInfo) -> Self {
        let buffer_local = TypedUniformBuffer::from_def(info.props.clone());
        let buffer = res.create_buffer(&wgpu::BufferDescriptor {
            label:None,
            size:info.props.size() as u64,
            usage:wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM,
            mapped_at_creation:false
        });
        let mut textures = vec![];
        for _ in info.textures.iter() {
            textures.push(res.default_textures[0].clone_weak());
        }
        let layout = info.create_layout(&res.device);
        UniformObject {
            local_buffer: buffer_local,
            buffer,
            cache_buffer:None,
            layout,
            bind_group:None,
            texture_dirty:true,
            textures
        }
    }

    fn update_bind_group(&mut self,res:&RenderResources)  {
        if !self.texture_dirty || !res.is_textures_ready(&self.textures)   { return };
        
        let mut builder = BindGroupBuilder::new();
        builder.add_buffer(self.buffer);
        for texture in self.textures.iter() {
            builder.add_texture(texture.clone());
        }
        let bind_group = builder.build(&self.layout, &res.device, &res);
        self.bind_group = Some(bind_group);
        log::error!("set bind_group {:?}",self.local_buffer.def.names());
        self.texture_dirty = false;
    }

    fn update_buffer(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
        if !self.local_buffer.is_dirty() { return; }

        let cache_id = match self.cache_buffer {
            Some(cache_id) => {
                let buffer_size = self.local_buffer.def.size() as u64;
                res.map_buffer(&cache_id, wgpu::MapMode::Write);
                res.write_mapped_buffer(&cache_id, 0.. buffer_size,&mut |bytes,_| {
                    bytes[0..buffer_size as usize].copy_from_slice(self.local_buffer.get_buffer());
                });
                res.unmap_buffer(&cache_id);
                cache_id
            },
            None => {
                let cache_id = res.create_buffer(&wgpu::BufferDescriptor {
                    label:None,
                    size:self.local_buffer.def.size() as u64,
                    usage:wgpu::BufferUsage::COPY_SRC | wgpu::BufferUsage::MAP_WRITE,
                    mapped_at_creation:false
                });
                self.cache_buffer = Some(cache_id);
                cache_id
            }
        };
        res.copy_buffer_to_buffer(cmd,
            &cache_id,
            0,
        &self.buffer,
        0, 
                    self.local_buffer.def.size() as u64);
        self.local_buffer.clear_dirty();

    }

    pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
        self.update_buffer(res,cmd);
        self.update_bind_group(res);
    }

   
}