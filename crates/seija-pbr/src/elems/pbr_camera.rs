use bevy_ecs::{prelude::{World, Entity}, query::{Or, Changed, Added}};
use lite_clojure_eval::{Variable, GcRefCell};
use seija_render::{dsl_frp::IUpdateNode, RenderContext, UniformIndex};
use anyhow::{Result,anyhow};

use crate::PBRCameraInfo;
pub struct PBRCameraNode {
    pub ubo_name:String,
    name_index:UniformIndex,
    exposure_index:usize
}

impl PBRCameraNode {
    pub fn from_args(args:Vec<Variable>) -> Result<Box<dyn IUpdateNode>> {
        let name:GcRefCell<String> = args.get(0).and_then(Variable::cast_string)
                                         .ok_or(anyhow!("type cast error"))?;
        let br_name = name.borrow().clone();
        Ok(Box::new(PBRCameraNode {
            ubo_name :br_name,
            name_index:UniformIndex::default(),
            exposure_index:0
        })) 
    }
}

impl IUpdateNode for PBRCameraNode {
    fn active(&mut self,_:&mut World,ctx:&mut RenderContext) -> Result<()> {
        self.name_index = ctx.ubo_ctx.get_index(self.ubo_name.as_str())
                                     .ok_or(anyhow!("not found ubo {}",&self.ubo_name))?;
        let info = ctx.ubo_ctx.info.get_info(&self.ubo_name)
                                                 .ok_or(anyhow!("not found ubo {}",&self.ubo_name))?;
        self.exposure_index = info.props.get_offset("exposure", 0)
                                        .ok_or(anyhow!("not found exposure"))?;
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        let mut cameras = world.query_filtered::<(Entity,&PBRCameraInfo),Or<(Changed<PBRCameraInfo>,Added<PBRCameraInfo>)>>();
        for (e,ex_info) in cameras.iter(world) {
            ctx.ubo_ctx.set_buffer(&self.name_index, Some(e.id()),|buffer| {
                buffer.buffer.write_bytes(self.exposure_index, ex_info.exposure.exposure_self());
            })
        }
        Ok(())
    }

}
