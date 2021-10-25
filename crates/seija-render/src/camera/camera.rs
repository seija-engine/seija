use std::collections::HashMap;

use bevy_ecs::prelude::{Changed, Entity, Local, Mut, RemovedComponents, World};
use glam::Mat4;
use seija_core::bytes::AsBytes;
use seija_transform::Transform;
use wgpu::{BindingResource, Buffer, BufferUsage, Device};
use crate::MATRIX_SIZE;
use crate::resource::{BufferId, RenderResources};
use crate::render::RenderContext;

use super::view_list::ViewList;

pub enum Projection {
    Ortho(Orthographic)
}

impl Projection {
    pub fn matrix(&self) -> Mat4 {
        match self {
            Projection::Ortho(o) => o.proj_matrix(),
        }
    }
}

pub struct Orthographic {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Orthographic {
    fn default() -> Self {
        Self { 
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1000.0,
         }
    }
}

impl Orthographic {
    pub fn proj_matrix(&self) -> Mat4  {
        Mat4::orthographic_rh(self.left,self.right,self.bottom,self.top,self.near,self.far)
    } 
}


pub struct Camera {
   pub projection:Projection,
   pub view_list:ViewList,
}

#[derive(Default)]
pub struct CameraState {
    cameras_buffer:CamerasBuffer
}

impl CameraState {
    pub fn init(ctx:&mut RenderContext) {
        
    }
}

pub struct CameraBuffer {
    pub staging_buffer:Option<Buffer>,
    pub view_proj:Buffer,
    pub view:Buffer
}

#[derive(Default)]
pub struct CamerasBuffer {
    pub buffers:HashMap<u32,CameraBuffer>
}

impl Camera {
    pub fn from_2d(ortho:Orthographic) -> Camera {
        Camera { projection:Projection::Ortho(ortho),view_list:ViewList::default() }
    }
}

impl CamerasBuffer {
    pub fn get_or_create_buffer(&mut self,eid:u32,device:&Device) -> &mut CameraBuffer {
        if !self.buffers.contains_key(&eid) {
            let view_proj = device.create_buffer(&wgpu::BufferDescriptor {
                label:None,
                size:MATRIX_SIZE,
                usage:BufferUsage::COPY_DST | BufferUsage::UNIFORM,
                mapped_at_creation:false
            });
            let view = device.create_buffer(&wgpu::BufferDescriptor {
                label:None,
                size:MATRIX_SIZE,
                usage:BufferUsage::COPY_DST | BufferUsage::UNIFORM,
                mapped_at_creation:false
            });
            device.create_bind_group(desc)
            self.buffers.insert(eid,CameraBuffer {
                staging_buffer:None,
                view_proj,
                view
            });
        }
        self.buffers.get_mut(&eid).unwrap()
    }
}


pub(crate) fn update_camera(world:&mut World,ctx:&mut RenderContext) {
    let mut camera_query = world.query::<(Entity,&Transform, &Camera)>();
    for (e,t,camera) in camera_query.iter(world) {
        let buffer = ctx.camera_state.cameras_buffer.get_or_create_buffer(e.id(),&ctx.device);
        if let Some(staging_buffer) = buffer.staging_buffer.as_ref() {
           {
                let buffer_slice = staging_buffer.slice(..);
                let data = buffer_slice.map_async(wgpu::MapMode::Write);
                ctx.device.poll(wgpu::Maintain::Wait);
                if futures_lite::future::block_on(data).is_err() {
                    panic!("Failed to map buffer to host.");
                };
           }
        } else {
            let staging_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label:None,
                size:MATRIX_SIZE * 2,
                usage:BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
                mapped_at_creation:true
            });
            buffer.staging_buffer = Some(staging_buffer);
        }

        let command = ctx.command_encoder.as_mut().unwrap();
        
        let staging_buffer = buffer.staging_buffer.as_ref().unwrap();
        {
            let view_proj_matrix = t.global().matrix().inverse() * camera.projection.matrix();
            let buffer_slice = staging_buffer.slice(0..MATRIX_SIZE * 2);
            let mut data = buffer_slice.get_mapped_range_mut();
            data[0..crate::MATRIX_SIZE as usize].copy_from_slice(view_proj_matrix.to_cols_array_2d().as_bytes());

            let view_matrix = t.global().matrix();
            data[(MATRIX_SIZE as usize) ..(MATRIX_SIZE*2) as usize].copy_from_slice(view_matrix.to_cols_array_2d().as_bytes());

            command.copy_buffer_to_buffer(staging_buffer, 0, 
                                      &buffer.view_proj, 0, MATRIX_SIZE);
            command.copy_buffer_to_buffer(staging_buffer, MATRIX_SIZE,
                                      &buffer.view_proj, 0, MATRIX_SIZE);
        }
        
        
        staging_buffer.unmap();
    }
    for e in world.removed::<Camera>() {
        ctx.camera_state.cameras_buffer.buffers.remove(&e.id());
    }
    
}