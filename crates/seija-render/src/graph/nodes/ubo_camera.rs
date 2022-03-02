use crate::{RenderContext, graph::node::INode, memory::UniformBufferDef};
use bevy_ecs::prelude::*;
use crate::resource::RenderResourceId;
pub struct UBOCamera {
    buffer:UniformBufferDef
}

impl INode for UBOCamera {
    fn update(&mut self,_world: &mut World,_render_ctx:&mut RenderContext,_inputs:&Vec<Option<RenderResourceId>>,_outputs:&mut Vec<Option<RenderResourceId>>) {
       
    }
}
