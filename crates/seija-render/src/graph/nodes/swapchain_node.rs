use bevy_ecs::prelude::World;

use crate::{graph::node::INode, render::RenderContext, resource::RenderResourceId};

pub struct SwapchainNode {

}

impl SwapchainNode {
    pub fn new() -> SwapchainNode {
        SwapchainNode {}
    }
}

impl INode for SwapchainNode {
    fn update(&mut self,world: &mut World,
                        ctx:&mut RenderContext,
                        inputs:&Vec<Option<RenderResourceId>>,
                        outputs:&mut Vec<Option<RenderResourceId>>) {
                            
    }

    fn output_count(&self) -> usize { 1 }
}