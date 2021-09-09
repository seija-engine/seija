use std::collections::HashMap;

use bevy_ecs::prelude::{Changed, Entity, Mut, RemovedComponents, World};
use glam::Mat4;
use seija_core::bytes::AsBytes;
use seija_transform::Transform;
use wgpu::BufferUsage;
use crate::resource::BufferId;
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

pub struct  CameraBuffer {
    pub view_proj:BufferId
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

pub(crate) fn update_camera(w:&mut World,ctx:&mut RenderContext) {
    w.resource_scope(|world,mut buffers:Mut<CamerasBuffer>| {
        {
            let mut camera_query = world.query::<(Entity,&Transform, &Camera)>();
            for (e,t,camera) in camera_query.iter(world) {
                let view_proj_matrix = t.global().matrix().inverse() * camera.projection.matrix();
                let view_proj_buffer_id = if let Some(camera_buffer) = buffers.buffers.get_mut(&e.id()) {
                    ctx.resources.map_buffer(camera_buffer.view_proj, wgpu::MapMode::Write);
                    camera_buffer.view_proj
                } else {
                    let buffer_id = ctx.resources.create_buffer(&wgpu::BufferDescriptor {
                        label:None,
                        size:crate::MATRIX_SIZE,
                        usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
                        mapped_at_creation:true
                    });
                    let new_buffer = CameraBuffer {view_proj : buffer_id};
                    buffers.buffers.insert(e.id(), new_buffer);
                    buffer_id
                };
                ctx.resources.write_mapped_buffer(view_proj_buffer_id, 0..crate::MATRIX_SIZE, &mut |data,_| {
                    data[0..crate::MATRIX_SIZE as usize].copy_from_slice(view_proj_matrix.to_cols_array_2d().as_bytes());
                });
                ctx.resources.unmap_buffer(view_proj_buffer_id);
            }
        };
        
        for e in world.removed::<Camera>() {
            let buffer = buffers.buffers.remove(&e.id()).unwrap();
            ctx.resources.remove_buffer(buffer.view_proj);
        }
        
    });
  


     
}