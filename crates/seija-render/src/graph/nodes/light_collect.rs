use bevy_ecs::prelude::World;

use crate::{uniforms::{UBONameIndex, backends::LightBackend}, graph::node::INode, RenderContext, resource::RenderResourceId};

#[derive(Default)]
pub struct LightCollect {
   pub ubo_name:String,
   name_index:Option<UBONameIndex>,
   backend:Option<LightBackend>
}

impl INode for LightCollect {
    
    fn init(&mut self, _world: &mut World, ctx:&mut RenderContext) {
        if let Some(info) = ctx.ubo_ctx.info.get_info(&self.ubo_name) {
            match LightBackend::from_def(&info.props) {
                Ok(backend) => {
                    self.backend = Some(backend)
                },
                Err(err) => {
                    log::error!("LightBackend backend error :{}",err);
                }
            }
            self.name_index = Some(ctx.ubo_ctx.buffers.get_name_index(self.ubo_name.as_str()).unwrap())
         }
    }

    fn prepare(&mut self, _world: &mut World, ctx:&mut RenderContext) {
        if let Some(type_ubo) = self.name_index.and_then(|index| ctx.ubo_ctx.buffers.get_buffer_mut(&index, None) ) {
            if let Some(backend) = self.backend.as_ref() {
               // backend.set_lights_type(&mut type_ubo.buffer, 0, 6);
            }
        }
    }

    fn update(&mut self,world: &mut World,render_ctx:&mut RenderContext,inputs:&Vec<Option<RenderResourceId>>,outputs:&mut Vec<Option<RenderResourceId>>) {
      
    }
}