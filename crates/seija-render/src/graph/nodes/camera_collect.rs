use std::collections::HashMap;
use crate::camera::camera::{Camera};
use crate::uniforms::{BufferIndex, UBObject};
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
   cameras_ubo:HashMap<u32,BufferIndex>,
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
                  log::error!("camera3d backend error :{}",err);
              }
          }
       }
    }

    fn prepare(&mut self, world: &mut World,ctx:&mut RenderContext) {
        let mut added_cameras = world.query_filtered::<Entity,(Added<Camera>,With<Transform>)>(); 
        for v in added_cameras.iter(&world) {
           if let Some(key) = ctx.ubo_ctx.add_camera_buffer(self.ubo_name.as_str(), v.id(), &mut ctx.resources) {
               self.cameras_ubo.insert(v.id(), key);     
           } else {
               log::error!("add {} ubo error",&self.ubo_name);
           }
        }
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,_:&Vec<Option<RenderResourceId>>,_:&mut Vec<Option<RenderResourceId>>) {
        let mut cameras = world.query::<(Entity,&Transform,&Camera)>();
        for (e,t,camera) in cameras.iter(world) {
            if let Some(key) = self.cameras_ubo.get(&e.id()) {
               if let Some(ubo) = ctx.ubo_ctx.buffers.get_camera_mut(&key) {
                   self.update_camera_buffer(ubo,t, camera);
               }
            }
        }
    }
}

impl CameraCollect {
    fn update_camera_buffer(&self,ubo:&mut UBObject,t:&Transform,camera:&Camera) {
        if let Some(backend) = self.backend.as_ref() {
            let proj = camera.projection.matrix();
            let proj_view = t.global().matrix().inverse() * proj;
            let view = t.global().matrix().inverse();
            let v3 = t.global().position;
            let pos = Vec4::new(v3.x,v3.y,v3.z,0f32);
            let buffer = &mut ubo.local.buffer;

            backend.set_view(buffer, &view);
            backend.set_proj(buffer, &proj);
            backend.set_projview(buffer, &proj_view);
            backend.set_position(buffer, pos);
        }
    }
}
