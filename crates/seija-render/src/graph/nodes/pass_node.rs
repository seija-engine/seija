use crate::{graph::node::INode, render::{RenderContext, RenderGraphContext}};
use bevy_ecs::prelude::*;
use crate::resource::ResourceId;
pub struct PassNode;

impl INode for PassNode {
    fn update(&mut self,_world: &mut World,ctx:&mut RenderContext,_inputs:&Vec<Option<ResourceId>>,_outputs:&mut Vec<Option<ResourceId>>) {
        
    }
}