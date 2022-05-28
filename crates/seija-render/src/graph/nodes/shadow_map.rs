use bevy_ecs::prelude::World;

use crate::{graph::INode, resource::RenderResourceId};

pub struct ShadowMapNode {

}

impl INode for ShadowMapNode {
    fn update(&mut self,world: &mut World,
              ctx:&mut crate::RenderContext,
              inputs:&Vec<Option<RenderResourceId>>,
              outputs:&mut Vec<Option<RenderResourceId>>) {
       
    }
}