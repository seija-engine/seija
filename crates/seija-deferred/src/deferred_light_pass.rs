use bevy_ecs::prelude::World;
use seija_render::{graph::INode, RenderContext, resource::RenderResourceId};

pub struct DeferredLightPass {
    tex_count:usize
}

impl DeferredLightPass {
    pub fn new(tex_count:usize) -> Self {
        DeferredLightPass {
            tex_count
        }
    }
}

impl INode for DeferredLightPass {
    fn init(&mut self, _world: &mut World, ctx:&mut RenderContext) {
        
    }

    fn prepare(&mut self, _world: &mut World, ctx:&mut RenderContext) {
        
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,
              inputs:&Vec<Option<RenderResourceId>>,
              outputs:&mut Vec<Option<RenderResourceId>>) {
       
    }
}