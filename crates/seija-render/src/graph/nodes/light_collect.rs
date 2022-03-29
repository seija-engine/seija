use bevy_ecs::prelude::World;
use glam::Vec4;
use crate::light::{LightEnv};
use crate::{uniforms::{UBONameIndex, backends::LightBackend}, graph::node::INode, RenderContext, resource::RenderResourceId, memory::TypedUniformBuffer};

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

    fn prepare(&mut self, world: &mut World, ctx:&mut RenderContext) {
        
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,inputs:&Vec<Option<RenderResourceId>>,outputs:&mut Vec<Option<RenderResourceId>>) {
        self._update(world,  ctx);
    }
}


impl LightCollect {
    pub fn _update(&mut self,world:&mut World,ctx:&mut RenderContext) -> Option<()> {
        let type_ubo = self.name_index.and_then(|index| ctx.ubo_ctx.buffers.get_buffer_mut(&index, None))?;
        let backend = self.backend.as_ref()?;
        if let Some(mut light_env) = world.get_resource_mut::<LightEnv>() {
            if light_env.is_dirty {
                backend.set_ambile_color(&mut type_ubo.buffer, light_env.ambient_color);
                light_env.clear_dirty();
            }
        }

        
        None
    }
}