use bevy_ecs::prelude::{World, Entity, Changed, Or, Added};
use lite_clojure_eval::Variable;
use anyhow::{Result,anyhow};
use seija_render::{IUpdateNode, RenderContext, UniformIndex};

use crate::PBRCameraInfo;
#[derive(Default)]
pub struct PBRCameraNode {
    pub ubo_name:String,
    name_index:UniformIndex,
    exposure_index:usize
}

impl IUpdateNode for PBRCameraNode {
    fn update_params(&mut self,params:Vec<Variable>) {
        if let Some(string) = params.get(0).and_then(Variable::cast_string) {
            self.ubo_name = string.borrow().clone();
        }
    }

    fn init(&mut self,_:& World,ctx:&mut RenderContext) -> Result<()> {
        self.name_index = ctx.ubo_ctx.get_index(self.ubo_name.as_str()).ok_or(anyhow!("not found ubo {}",&self.ubo_name))?;
        let info = ctx.ubo_ctx.info.get_info(&self.ubo_name).ok_or(anyhow!("not found ubo {}",&self.ubo_name))?;
        self.exposure_index = info.props.get_offset("exposure", 0).ok_or(anyhow!("not found exposure"))?;
        
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
        let mut cameras = world.query_filtered::<(Entity,&PBRCameraInfo),Or<(Changed<PBRCameraInfo>,Added<PBRCameraInfo>)>>();
        for (e,ex_info) in cameras.iter(world) {
            ctx.ubo_ctx.set_buffer(&self.name_index, Some(e.id()),|buffer| {
                buffer.buffer.write_bytes(self.exposure_index, ex_info.exposure.exposure_self());
            })
        }
    }
}