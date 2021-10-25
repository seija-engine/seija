use std::collections::HashMap;

use bevy_ecs::prelude::{Changed, Entity, Local, Mut, RemovedComponents, World};
use glam::Mat4;
use seija_core::bytes::AsBytes;
use seija_transform::Transform;
use wgpu::{BindingResource, Buffer, BufferUsage, Device, ShaderStage};
use crate::MATRIX_SIZE;
use crate::pipeline::render_bindings::RenderBindings;
use crate::resource::{BufferId, RenderResourceId, RenderResources};
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
    pub bindings:RenderBindings,
    pub staging_buffer:Option<BufferId>,
    pub view_proj:BufferId,
    pub view:BufferId
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
    pub fn get_or_create_buffer(&mut self,eid:u32,device:&Device,resources:&mut RenderResources) -> &mut CameraBuffer {
        if !self.buffers.contains_key(&eid) {
            let view_proj = resources.create_buffer(&wgpu::BufferDescriptor {
                label:None,
                size:MATRIX_SIZE,
                usage:BufferUsage::COPY_DST | BufferUsage::UNIFORM,
                mapped_at_creation:false
            });
            let view = resources.create_buffer(&wgpu::BufferDescriptor {
                label:None,
                size:MATRIX_SIZE,
                usage:BufferUsage::COPY_DST | BufferUsage::UNIFORM,
                mapped_at_creation:false
            });
            let mut bindings = RenderBindings::default();
    
            bindings.add_uniform(ShaderStage::VERTEX, view_proj);
            bindings.add_uniform(ShaderStage::VERTEX, view);
            bindings.build(device,resources);
            self.buffers.insert(eid,CameraBuffer {
                bindings,
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
        let buffer = ctx.camera_state.cameras_buffer.get_or_create_buffer(e.id(), &ctx.device,&mut ctx.resources);
        if let Some(staging_buffer) = buffer.staging_buffer {
           {
                ctx.resources.map_buffer(staging_buffer, wgpu::MapMode::Write);
           }
        } else {
            let staging_buffer = ctx.resources.create_buffer(&wgpu::BufferDescriptor {
                label:None,
                size:MATRIX_SIZE * 2,
                usage:BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
                mapped_at_creation:true
            });
            buffer.staging_buffer = Some(staging_buffer);
        }

        let command = ctx.command_encoder.as_mut().unwrap();
        
        let staging_buffer = buffer.staging_buffer.unwrap();
        {
            let view_proj_matrix = t.global().matrix().inverse() * camera.projection.matrix();
            let view_matrix = t.global().matrix();
            ctx.resources.write_mapped_buffer(staging_buffer, 0..(MATRIX_SIZE * 2),&mut |bytes,_| {
                bytes[0..crate::MATRIX_SIZE as usize].copy_from_slice(view_proj_matrix.to_cols_array_2d().as_bytes());
                bytes[(MATRIX_SIZE as usize) ..(MATRIX_SIZE*2) as usize].copy_from_slice(view_matrix.to_cols_array_2d().as_bytes());
            });
            
            ctx.resources.copy_buffer_to_buffer(command, staging_buffer,0, buffer.view_proj,0, MATRIX_SIZE);
            ctx.resources.copy_buffer_to_buffer(command, staging_buffer,MATRIX_SIZE, buffer.view,0, MATRIX_SIZE);
        }
        
        
        ctx.resources.unmap_buffer(staging_buffer);
    }
    for e in world.removed::<Camera>() {
        ctx.camera_state.cameras_buffer.buffers.remove(&e.id());
    }
    
}