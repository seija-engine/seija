use crate::{RenderContext, graph::node::INode, memory::UniformBufferDef};
use bevy_ecs::prelude::*;
use crate::resource::RenderResourceId;
pub struct CameraCollect {
  pub ubo_name:String
}

impl INode for CameraCollect {
    
    fn prepare(&mut self, _world: &mut World,ctx:&mut RenderContext) {
        
    }

    fn update(&mut self,_world: &mut World,_:&mut RenderContext,_:&Vec<Option<RenderResourceId>>,_:&mut Vec<Option<RenderResourceId>>) {
       
    }
}
