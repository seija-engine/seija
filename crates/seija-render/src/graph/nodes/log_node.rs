use crate::graph::node::INode;
use bevy_ecs::prelude::*;
use crate::resource::ResourceId;
pub struct LogNode(pub String,pub usize,pub usize);

impl INode for LogNode {
    fn input_count(&self)  -> usize { self.1 }
    fn output_count(&self) -> usize { self.2 }

    fn update(&mut self,_world: &mut World,_inputs:&Vec<Option<ResourceId>>,_outputs:&mut Vec<Option<ResourceId>>) {
        match  self.0.as_str()  {
            "a" => {
                _outputs[0] = Some(ResourceId(1))
            },
            "b" => {
                _outputs[0] = Some(ResourceId(2))
            },
            "c" => {
                dbg!(_inputs);
                _outputs[0] = Some(ResourceId(3))
            },
            "d" => {
                _outputs[0] = _inputs[0].clone();
            },
            _ => {}
        }
    }
}