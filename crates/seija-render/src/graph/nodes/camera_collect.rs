use std::collections::HashMap;
use crate::camera::camera::{Camera};
use crate::memory::TypedUniformBuffer;
use crate::uniforms::{UBONameIndex, UBObject};
use crate::uniforms::backends::Camera3DBackend;
use crate::{RenderContext, graph::node::INode};
use bevy_ecs::prelude::*;
use glam::Vec4;
use seija_transform::Transform;
use crate::resource::RenderResourceId;

//TODO 没有实现移除逻辑
#[derive(Default)]
pub struct CameraCollect {
   pub ubo_name:String,
   name_index:Option<UBONameIndex>,
   backend:Option<Camera3DBackend>
}

impl INode for CameraCollect {
  
    fn init(&mut self, _world: &mut World,ctx:&mut RenderContext) {
       if let Some(info) = ctx.ubo_ctx.info.get_info(&self.ubo_name) {
          match Camera3DBackend::from_def(&info.props) {
              Ok(backend) => {
                  self.backend = Some(backend)
              },
              Err(err) => {
                  log::error!("Camera3DBackend backend error :{}",err);
              }
          }
          self.name_index = Some(ctx.ubo_ctx.buffers.get_name_index(self.ubo_name.as_str()).unwrap())
       }
       
    }

    fn prepare(&mut self, world: &mut World,ctx:&mut RenderContext) {
        let mut added_cameras = world.query_filtered::<Entity,(Added<Camera>,With<Transform>)>(); 
        for v in added_cameras.iter(&world) {
            ctx.ubo_ctx.add_buffer(self.ubo_name.as_str(), &mut ctx.resources,Some(v.id()));
        }
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,_:&Vec<Option<RenderResourceId>>,_:&mut Vec<Option<RenderResourceId>>) {
        let mut cameras = world.query::<(Entity,&Transform,&Camera)>();
        for (e,t,camera) in cameras.iter(world) {
            if let Some(key) = self.name_index {
                if let Some(buffer) = ctx.ubo_ctx.buffers.get_buffer_mut(&key, Some(e.id())) {
                    self.update_camera_buffer(buffer,t, camera);
                }
            }
        }
    }
}

impl CameraCollect {
    fn update_camera_buffer(&self,buffer:&mut TypedUniformBuffer,t:&Transform,camera:&Camera) {
        if let Some(backend) = self.backend.as_ref() {
            let proj = camera.projection.matrix();
            let proj_view = t.global().matrix().inverse() * proj;
            let view = t.global().matrix().inverse();
            let v3 = t.global().position;
            let pos = Vec4::new(v3.x,v3.y,v3.z,0f32);
            let buffer = &mut buffer.buffer;

            backend.set_view(buffer, &view);
            backend.set_proj(buffer, &proj);
            backend.set_projview(buffer, &proj_view);
            backend.set_position(buffer, pos);
        }
    }
}
