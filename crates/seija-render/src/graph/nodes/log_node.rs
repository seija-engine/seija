use crate::graph::node::INode;
use bevy_ecs::prelude::*;
use crate::resource::ResourceId;
pub struct LogNode(pub String);

impl INode for LogNode {
    fn update(&mut self,_world: &World,_inputs:&Vec<Option<ResourceId>>,_outputs:&mut Vec<Option<ResourceId>>) {
         println!("run {}",self.0.as_str());
    }
}