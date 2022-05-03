use bevy_ecs::prelude::World;

use crate::{RenderContext, graph::node::INode, resource::{RenderResourceId}};

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
       if let Ok(res_id) = ctx.resources.next_swap_chain_texture() {
           outputs[0] = Some(res_id);
       }
    }

    fn output_count(&self) -> usize { 2 }
}