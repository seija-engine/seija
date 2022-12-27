use std::collections::HashMap;

use seija_asset::Handle;
use smol_str::SmolStr;
use wgpu::CommandEncoder;

use crate::{
    resource::{RenderResources, BufferId, Texture}, 
    UniformInfo, memory::TypedUniformBuffer, pipeline::render_bindings::BindGroupBuilder};

pub struct UniformObject {
    //buffer
    pub local_buffer:TypedUniformBuffer,
    buffer:Option<BufferId>,
    cache_buffer:Option<BufferId>,
    //texture
    texture_idxs:HashMap<SmolStr,usize>,
    textures:Vec<Handle<Texture>>,
    texture_dirty:bool,
    pub layout:wgpu::BindGroupLayout,
    pub bind_group:Option<wgpu::BindGroup>
}

impl UniformObject {
    pub fn new(res:&mut RenderResources,info:&UniformInfo) -> Self {
        let buffer_local = TypedUniformBuffer::from_def(info.props.clone());
        
        let buffer = if !buffer_local.is_empty() {
            Some(res.create_buffer(&wgpu::BufferDescriptor {
                label:None,
                size:buffer_local.def.size() as u64,
                usage:wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                mapped_at_creation:false
            }))
        } else { None };


        let cache_buffer = if !buffer_local.is_empty() {
            Some(res.create_buffer(&wgpu::BufferDescriptor {
                label:None,
                size:buffer_local.def.size() as u64,
                usage:wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_WRITE,
                mapped_at_creation:false
            }))
        } else { None };

        let mut texture_idxs:HashMap<SmolStr,usize> = HashMap::default();
        let mut textures = vec![];
        for (index,def) in info.textures.iter().enumerate() {
            if def.str_type != "texture2DArray" {
                if def.is_cubemap {
                    textures.push(res.default_textures[1].clone_weak());
                } else {
                    textures.push(res.default_textures[0].clone_weak());
                }
                texture_idxs.insert(def.name.as_str().into(), index);
            }
        }
       
        let layout = info.create_layout(&res.device);
        UniformObject {
            texture_idxs,
            local_buffer: buffer_local,
            buffer,
            cache_buffer,
            layout,
            bind_group:None,
            texture_dirty:true,
            textures
        }
    }


    fn update_bind_group(&mut self,res:&RenderResources)  {
        if !self.texture_dirty || !res.is_textures_ready(&self.textures)   { return };
        let mut builder = BindGroupBuilder::new();
        if let Some(buffer) = self.buffer {
            builder.add_buffer(buffer);
        }
        for texture in self.textures.iter() {
            builder.add_texture(texture.clone());
        }
        if !builder.is_empty() {
            let bind_group = builder.build(&self.layout, &res.device, &res);
            self.bind_group = Some(bind_group);
        }
        self.texture_dirty = false;
       
    }

    pub fn set_texture(&mut self,name:&str,texture:Handle<Texture>) {
        if let Some(index) = self.texture_idxs.get(name) {
            self.textures[*index] = texture;
            self.texture_dirty = true;
        } 
    }

    fn update_buffer(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
        if !self.local_buffer.is_dirty() { return; }
        if let (Some(cache_id),Some(buffer)) = (self.cache_buffer,self.buffer) {
            let buffer_size = self.local_buffer.def.size() as u64;
            res.map_buffer(&cache_id, wgpu::MapMode::Write);
            res.write_mapped_buffer(&cache_id, 0.. buffer_size,&mut |bytes,_| {
               
                bytes[0..buffer_size as usize].copy_from_slice(self.local_buffer.get_buffer());
            });
            res.unmap_buffer(&cache_id);
    
            res.copy_buffer_to_buffer(cmd,
                &cache_id,
                0,
            &buffer,
            0, 
                        self.local_buffer.def.size() as u64);
            
        }
        self.local_buffer.clear_dirty();

    }

    pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
        self.update_buffer(res,cmd);
        self.update_bind_group(res);
    }

   
}