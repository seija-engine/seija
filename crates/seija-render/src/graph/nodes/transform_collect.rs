use crate::{graph::node::INode, RenderContext, resource::{RenderResourceId, Mesh}, uniforms::{backends::TransformBackend}, material::Material, UniformIndex};
use bevy_ecs::prelude::*;
use seija_asset::Handle;
use seija_transform::Transform;

#[derive(Default)]
pub struct TransformCollect {
   pub ubo_name:String,
   backend:Option<TransformBackend>,
   name_index:Option<UniformIndex>,
}


impl INode for TransformCollect {
    fn init(&mut self, _world: &mut World,ctx:&mut RenderContext) {
        if let Some(info) = ctx.ubo_ctx.info.get_info(&self.ubo_name) {
            match TransformBackend::from_def(&info.props) {
                Ok(backend) => {
                    self.backend = Some(backend)
                },
                Err(err) => {
                    log::error!("TransformBackend backend error :{}",err);
                }
            }
            if let Some(index) = ctx.ubo_ctx.get_index(self.ubo_name.as_str()) {
                self.name_index = Some(index);
            } else {
                log::error!("not found {}",self.ubo_name.as_str())
            }
         }
    }

    fn prepare(&mut self, world: &mut World,ctx:&mut RenderContext) {
        let mut added_transform = world.query_filtered::<Entity,(Added<Transform>,With<Handle<Mesh>>,With<Handle<Material>>)>();
        if let Some(name_index) = self.name_index {
            for v in added_transform.iter(&world) {
                ctx.ubo_ctx.add_component(&name_index,v.id(),&mut ctx.resources)
            }
    
            for rm_e in world.removed::<Transform>() {
               ctx.ubo_ctx.remove_component(&name_index, rm_e.id());
            }
        }
       

    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,_:&Vec<Option<RenderResourceId>>,_:&mut Vec<Option<RenderResourceId>>) {
        let mut trans = world.query_filtered::<(Entity,&Transform),(Changed<Transform>,With<Handle<Mesh>>,With<Handle<Material>>)>();
        for (e,t) in trans.iter(world) { 
           
            if let Some(key) = self.name_index {
                if let Some(backend) = self.backend.as_ref() {
                    ctx.ubo_ctx.set_buffer(&key, Some(e.id()), |buffer| {
                        backend.set_transform(&mut buffer.buffer,  &t.global().matrix());
                    });  
                }
            }
        }
    }
}