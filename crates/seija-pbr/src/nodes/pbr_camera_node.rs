use bevy_ecs::prelude::{World, Entity, With, Changed};
use lite_clojure_eval::Variable;
use seija_render::{IUpdateNode, RenderContext, UniformIndex, camera::camera::Camera};
use seija_transform::Transform;

use crate::PBRCameraInfo;
#[derive(Default)]
pub struct PBRCameraNode {
    pub ubo_name:String,
    name_index:Option<UniformIndex>,
    exposure_index:Option<usize>
}

impl IUpdateNode for PBRCameraNode {
    fn update_params(&mut self,params:Vec<Variable>) {
        if let Some(string) = params.get(0).and_then(Variable::cast_string) {
            self.ubo_name = string.borrow().clone();
        }
    }

    fn init(&mut self,_:& World,ctx:&mut RenderContext) {
        self.name_index = ctx.ubo_ctx.get_index(self.ubo_name.as_str());
        if let Some(info) = ctx.ubo_ctx.info.get_info(&self.ubo_name) {
            if let Some(idx) = info.props.get_offset("exposure", 0) {
                self.exposure_index = Some(idx);
            } else {
                self.exposure_index = None;
                log::error!("not found exposure in {}",self.ubo_name);
            }
        }
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
        if let Some(exposure_index) = self.exposure_index {
            let mut cameras = world.query_filtered::<(Entity,&PBRCameraInfo),(With<Camera>,With<Transform>,Changed<PBRCameraInfo>)>();
            for (e,ex_info) in cameras.iter(world) {
                if let Some(key) = self.name_index {
                    ctx.ubo_ctx.set_buffer(&key, Some(e.id()),|buffer| {
                        buffer.buffer.write_bytes(exposure_index, ex_info.exposure.exposure_self());
                    })
                    
                }
            }
        }
    }
}