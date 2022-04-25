use bevy_ecs::prelude::World;
use seija_render::{graph::INode, RenderContext, resource::RenderResourceId};

pub struct SkeletonNode {

}

impl INode for SkeletonNode {
    fn update(&mut self,world: &mut World,render_ctx:&mut RenderContext,inputs:&Vec<Option<RenderResourceId>>,outputs:&mut Vec<Option<RenderResourceId>>) {
        
    }
}