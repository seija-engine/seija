use crate::{graph::node::INode, RenderContext, resource::RenderResourceId, uniforms::{backends::TransformBackend, BufferArrayIndex}};
use bevy_ecs::prelude::*;
use fnv::FnvHashMap;
use seija_transform::Transform;
//TODO 没有实现移除逻辑
#[derive(Default)]
pub struct TransformCollect {
   pub ubo_name:String,
   backend:Option<TransformBackend>,
   trans_map:FnvHashMap<u32,BufferArrayIndex>
}


impl INode for TransformCollect {
    fn init(&mut self, _world: &mut World,ctx:&mut RenderContext) {
        if let Some(info) = ctx.ubo_ctx.info.get_info(&self.ubo_name) {
            match TransformBackend::from_def(&info.props) {
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
        let mut added_transform = world.query_filtered::<Entity,Added<Transform>>();
        for v in added_transform.iter(&world) {
          if let Some(array_index)  = ctx.ubo_ctx.add_object_buffer(&self.ubo_name,v.id(),&mut ctx.resources) {
              self.trans_map.insert(v.id(), array_index);
          }
        }
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,_:&Vec<Option<RenderResourceId>>,_:&mut Vec<Option<RenderResourceId>>) {
        let mut cameras = world.query_filtered::<(Entity,&Transform),Changed<Transform>>();
        for (e,t) in cameras.iter(world) { 
            if let Some(key) = self.trans_map.get(&e.id()) {
                if let Some(buffer) = ctx.ubo_ctx.buffers.get_object_mut(&key) {
                    if let Some(backend) = self.backend.as_ref() {
                        backend.set_transform(&mut buffer.buffer,  &t.global().matrix());
                    }
                }
            }
        }
    }
}