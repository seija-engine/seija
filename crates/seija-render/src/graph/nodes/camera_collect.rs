use std::collections::HashMap;
use std::{convert::TryFrom, sync::Arc};
use crate::camera::camera::{Camera};
use crate::memory::TypedUniformBuffer;
use crate::{RenderContext, graph::node::INode, memory::UniformBufferDef};
use bevy_ecs::prelude::*;
use seija_transform::Transform;
use crate::resource::RenderResourceId;

#[derive(Default)]
pub struct CameraCollect {
   pub ubo_name:String,
   buffer_def:Option<Arc<UniformBufferDef>>,
   buffers:HashMap<u32,TypedUniformBuffer>,
}

impl INode for CameraCollect {
  
    fn init(&mut self, _world: &mut World,ctx:&mut RenderContext) {
       
    }

    fn prepare(&mut self, world: &mut World,ctx:&mut RenderContext) {
         let mut camera_query = world.query::<(Entity,&Camera)>();
         for (e,_) in camera_query.iter(world) {
             if !self.buffers.contains_key(&e.id()) {
                let typed_buffer = TypedUniformBuffer::from_def(self.buffer_def.as_ref().unwrap().clone());
                self.buffers.insert(e.id(), typed_buffer);
             }
         }
    }

    fn update(&mut self,world: &mut World,_:&mut RenderContext,_:&Vec<Option<RenderResourceId>>,_:&mut Vec<Option<RenderResourceId>>) {
       let mut camera_query = world.query::<(Entity,&Transform,&Camera)>();
       for (e,t,camera) in camera_query.iter(world) {
           if let Some(buffer) = self.buffers.get_mut(&e.id()) {
               let proj = camera.projection.matrix();
               let view_proj_matrix = t.global().matrix().inverse() * proj;
               let view_matrix = t.global().matrix().inverse();
               let mut pos_bytes:[f32;4] = [0f32,0f32,0f32,1f32];
               t.global().position.write_to_slice(&mut pos_bytes);
               
           }
       }
    }
}
