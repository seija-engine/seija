use crate::{graph::node::INode, RenderContext, resource::RenderResourceId};
use bevy_ecs::prelude::*;

#[derive(Default)]
pub struct TransformCollect {
   pub ubo_name:String,
}


impl INode for TransformCollect {
    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,inputs:&Vec<Option<RenderResourceId>>,outputs:&mut Vec<Option<RenderResourceId>>) {
        
    }
}