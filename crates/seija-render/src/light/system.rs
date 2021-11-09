use bevy_ecs::prelude::World;
use seija_core::bytes::AsBytes;
use wgpu::{ CommandEncoder};

use crate::{pipeline::render_bindings::{BindGroupBuilder, BindGroupLayoutBuilder}, resource::{BufferId, RenderResources}};

use super::light::LightEnv;


pub struct LightState {
    staging_buffer:Option<BufferId>,
    light_buffer:Option<BufferId>,
    pub layout:wgpu::BindGroupLayout,
    pub bind_group:Option<wgpu::BindGroup>
}

impl LightState {
    pub fn new(device:&wgpu::Device) -> Self {
        device.limits().max_bind_groups;
        let mut layout_builder = BindGroupLayoutBuilder::new();
        layout_builder.add_uniform(wgpu::ShaderStage::FRAGMENT);
        let layout = layout_builder.build(device);
        Self {
            staging_buffer:None,
            light_buffer:None,
            layout,
            bind_group:None
        }
    }
}

impl LightState {
    pub fn update(&mut self,world:&mut World,res:&mut RenderResources,command:&mut CommandEncoder,device:&wgpu::Device) {
        if let Some(light_env) = world.get_resource::<LightEnv>() {
            if !light_env.is_dirty {
                return;
            }
            let bytes = light_env.inner().as_bytes();
            if let Some(stageing_buffer) = self.staging_buffer.as_ref() {
                
                res.map_buffer(stageing_buffer, wgpu::MapMode::Write);
                res.write_mapped_buffer(stageing_buffer, 0..bytes.len() as u64,&mut |buffer:&mut [u8],_| {
                    buffer[0..bytes.len()].copy_from_slice(bytes);
                });
                 res.unmap_buffer(stageing_buffer);
            } else {
                let staging_buffer_id = res.create_buffer_with_data(wgpu::BufferUsage::COPY_SRC | wgpu::BufferUsage::MAP_WRITE, bytes);
                let light_buffer_id = res.create_buffer(&wgpu::BufferDescriptor { 
                    label:None,
                    size:bytes.len() as u64,
                    usage:wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM,
                    mapped_at_creation:false 
                });
                self.staging_buffer = Some(staging_buffer_id);
                self.light_buffer = Some(light_buffer_id);

                let mut bind_group_builder = BindGroupBuilder::new();
                bind_group_builder.add_buffer(light_buffer_id);
                self.bind_group = Some(bind_group_builder.build(&self.layout, device, res));
            }
            res.copy_buffer_to_buffer(
                command, 
                  self.staging_buffer.as_ref().unwrap(), 0, 
              self.light_buffer.as_ref().unwrap(),
              0, 
                          bytes.len() as u64);
                         
        }
    }

}