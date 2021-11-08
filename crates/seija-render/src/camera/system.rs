use std::{collections::HashMap, sync::Arc};

use bevy_ecs::prelude::{Entity, World};
use glam::{EulerRot, Quat};
use seija_core::bytes::AsBytes;
use seija_transform::Transform;
use wgpu::{BufferUsage, Device, ShaderStage};

use crate::{MATRIX_SIZE, RenderContext, pipeline::render_bindings::{BindGroupBuilder, BindGroupLayoutBuilder}, resource::{BufferId, RenderResourceId, RenderResources}};

use super::camera::Camera;

pub struct CameraState {
    pub cameras_buffer:CamerasBuffer,
    pub camera_layout:Arc<wgpu::BindGroupLayout>
}

impl CameraState {
    pub fn new(device:&Device) -> CameraState {
        let mut layout_builder = BindGroupLayoutBuilder::new();
        layout_builder.add_uniform(ShaderStage::VERTEX);

        CameraState {
            cameras_buffer:CamerasBuffer::default(),
            camera_layout:Arc::new(layout_builder.build(device))
        }
    }
    
}

pub struct CameraBuffer {
    pub bind_group:wgpu::BindGroup,
    pub staging_buffer:Option<BufferId>,
    pub uniform:BufferId
}

#[derive(Default)]
pub struct CamerasBuffer {
    pub buffers:HashMap<u32,CameraBuffer>
}


impl CamerasBuffer {
    pub fn get_or_create_buffer(&mut self,eid:u32,device:&Device,camera_layout:&Arc<wgpu::BindGroupLayout>,resources:&mut RenderResources) -> &mut CameraBuffer {
        if !self.buffers.contains_key(&eid) {
            let uniform = resources.create_buffer(&wgpu::BufferDescriptor {
                label:None,
                size:MATRIX_SIZE * 3,
                usage:BufferUsage::COPY_DST | BufferUsage::UNIFORM,
                mapped_at_creation:false
            });
            

            let mut bind_group_builder = BindGroupBuilder::new();
            bind_group_builder.add_buffer(uniform);
           
            let bind_group = bind_group_builder.build(camera_layout, device, resources);

           
            
            self.buffers.insert(eid,CameraBuffer {
                bind_group,
                staging_buffer:None,
                uniform
            });
        }

        self.buffers.get_mut(&eid).unwrap()        
    }
}


pub(crate) fn update_camera(world:&mut World,ctx:&mut RenderContext) {
    let mut camera_query = world.query::<(Entity,&Transform, &Camera)>();
  
    for (e,t,camera) in camera_query.iter(world) {
        let buffer = ctx.camera_state.cameras_buffer.get_or_create_buffer(e.id(), &ctx.device,&ctx.camera_state.camera_layout,&mut ctx.resources);
        if let Some(staging_buffer) = buffer.staging_buffer {
           {
                ctx.resources.map_buffer(&staging_buffer, wgpu::MapMode::Write);
           }
        } else {
            let staging_buffer = ctx.resources.create_buffer(&wgpu::BufferDescriptor {
                label:None,
                size:MATRIX_SIZE * 3,
                usage:BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
                mapped_at_creation:true
            });
            buffer.staging_buffer = Some(staging_buffer);
        }

        let command = ctx.command_encoder.as_mut().unwrap();
        
        let staging_buffer = buffer.staging_buffer.unwrap();
        {
            let proj = camera.projection.matrix();
            let view_proj_matrix = t.global().matrix().inverse() * proj;
            let view_matrix = t.global().matrix().inverse();
           
            ctx.resources.write_mapped_buffer(&staging_buffer, 0..(MATRIX_SIZE * 3),&mut |bytes,_| {
                bytes[0..crate::MATRIX_SIZE as usize].copy_from_slice(view_proj_matrix.to_cols_array_2d().as_bytes());
                bytes[(MATRIX_SIZE as usize) ..(MATRIX_SIZE*2) as usize].copy_from_slice(view_matrix.to_cols_array_2d().as_bytes());
                bytes[(MATRIX_SIZE*2) as usize .. (MATRIX_SIZE*3) as usize].clone_from_slice(proj.to_cols_array_2d().as_bytes());
            });
            
            ctx.resources.copy_buffer_to_buffer(command, &staging_buffer,0, &buffer.uniform,0, MATRIX_SIZE * 3);
           
        }
        
        
        ctx.resources.unmap_buffer(&staging_buffer);
    }
    for e in world.removed::<Camera>() {
        ctx.camera_state.cameras_buffer.buffers.remove(&e.id());
    }
    
}