use crate::graph::node::INode;
use bevy_ecs::prelude::*;
use crate::resource::ResourceId;
pub struct PassNode;

impl INode for PassNode {
    fn update(&mut self,_world: &mut World,_inputs:&Vec<Option<ResourceId>>,_outputs:&mut Vec<Option<ResourceId>>) {
       println!("pass node");
    }
}