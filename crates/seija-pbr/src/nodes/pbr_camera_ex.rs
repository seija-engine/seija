use bevy_ecs::prelude::{World, Entity, With, Or, Added, Changed};
use seija_render::{UBONameIndex, graph::INode, RenderContext, resource::RenderResourceId, camera::camera::Camera};
use seija_transform::Transform;

use crate::PBRCameraInfo;

#[derive(Default)]
pub struct PBRCameraEx {
   pub ubo_name:String,
   name_index:Option<UBONameIndex>,
   exposure_index:Option<usize>
}

impl INode for PBRCameraEx {
    
    fn init(&mut self, _world: &mut World,ctx:&mut RenderContext) {
        self.name_index = ctx.ubo_ctx.buffers.get_name_index(self.ubo_name.as_str());
        if let Some(info) = ctx.ubo_ctx.info.get_info(&self.ubo_name) {
            if let Some(idx) = info.props.get_offset("exposure", 0) {
                self.exposure_index = Some(idx);
            } else {
                self.exposure_index = None;
                log::error!("not found exposure in {}",self.ubo_name);
            }
        }
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,_:&Vec<Option<RenderResourceId>>,_:&mut Vec<Option<RenderResourceId>>) {
        if let Some(exposure_index) = self.exposure_index {
            let mut cameras = world.query_filtered::<(Entity,&PBRCameraInfo),(With<Camera>,With<Transform>,Changed<PBRCameraInfo>)>();
            for (e,ex_info) in cameras.iter(world) {
                if let Some(key) = self.name_index {
                    if let Some(buffer) = ctx.ubo_ctx.buffers.get_buffer_mut(&key, Some(e.id())) {
                        //println!("set exposure {}",ex_info.exposure.exposure_self());
                        buffer.buffer.write_bytes(exposure_index, ex_info.exposure.exposure_self());
                    }
                }
            }
        }
    }
}