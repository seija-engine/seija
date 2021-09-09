use crate::{graph::node::INode, render::{RenderContext, RenderGraphContext}};
use bevy_ecs::prelude::*;
use crate::resource::ResourceId;
pub struct LogNode(pub String,pub usize,pub usize);

impl INode for LogNode {
    fn input_count(&self)  -> usize { self.1 }
    fn output_count(&self) -> usize { self.2 }

    fn update(&mut self,_world: &mut World,render_ctx:&mut RenderContext,_inputs:&Vec<Option<ResourceId>>,_outputs:&mut Vec<Option<ResourceId>>) {
       
    }
}