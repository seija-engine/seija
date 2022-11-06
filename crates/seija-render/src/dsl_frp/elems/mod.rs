use bevy_ecs::world::World;
use anyhow::Result;
use crate::{RenderContext, resource::TextureDescInfo};
pub mod camera_node;
pub mod transform_node;
use super::frp_comp::IElement;
pub struct UniformElement {
    name:String
}

impl UniformElement {
    pub fn new(name:String) -> Self {
        UniformElement { name }
    }
}

impl IElement for UniformElement {
   
    fn active(&mut self,_:&mut World,ctx:&mut RenderContext) -> Result<()> {
       log::info!("UniformElem active:{}",self.name.as_str());
       ctx.ubo_ctx.add_uniform(&self.name, &mut ctx.resources);
       Ok(())
    }

    fn deactive(&mut self,_:&mut World,ctx:&mut RenderContext) -> Result<()> {
        ctx.ubo_ctx.remove_uniform(&self.name);
        Ok(())
    }

    
}

pub trait IUpdateNode {
    fn init(&mut self,_world:&mut World,_ctx:&mut RenderContext) -> Result<()> { Ok(()) }
    fn active(&mut self,_world:&mut World,_ctx:&mut RenderContext) -> Result<()> { Ok(()) }
    fn deactive(&mut self,_world:&mut World,_ctx:&mut RenderContext) -> Result<()> { Ok(()) }
    fn update(&mut self,_world:&mut World,_ctx:&mut RenderContext) -> Result<()> { Ok(()) }
}

pub struct ElementNode {
    node:Box<dyn IUpdateNode>
}

impl ElementNode {
    pub fn new(node:Box<dyn IUpdateNode>) -> ElementNode {
        ElementNode { node }
    }
}

impl IElement for ElementNode {
    fn init(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        self.node.init(world, ctx)
    }
    
    fn active(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
       self.node.active(world, ctx)
    }

    fn deactive(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
      self.node.deactive(world, ctx)
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        self.node.update(world, ctx)
    }
    
}

pub struct TextureElement {
    desc_info:TextureDescInfo
}

impl TextureElement {
    pub fn new(desc_info:TextureDescInfo) -> Self {
        TextureElement { desc_info }
    }
}

impl IElement for TextureElement {
    fn active(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        Ok(())
    }

    fn deactive(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        Ok(())
    }
}