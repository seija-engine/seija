use crate::{RenderContext, graph::node::INode};
use bevy_ecs::prelude::*;
use crate::resource::RenderResourceId;
pub struct LogNode(pub String,pub usize,pub usize);

impl INode for LogNode {
    fn input_count(&self)  -> usize { self.1 }
    fn output_count(&self) -> usize { self.2 }

    fn update(&mut self,_world: &mut World,_render_ctx:&mut RenderContext,_inputs:&Vec<Option<RenderResourceId>>,_outputs:&mut Vec<Option<RenderResourceId>>) {
       
    }
}