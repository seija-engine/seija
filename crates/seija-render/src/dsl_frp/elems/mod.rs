use crate::RenderContext;

use super::frp_comp::IElement;

pub struct UniformElem {
    name:String
}

impl UniformElem {
    pub fn new(name:String) -> Self {
        UniformElem { name }
    }
}

impl IElement for UniformElem {
    fn active(&mut self,ctx:&mut RenderContext) {
       log::info!("UniformElem active:{}",self.name.as_str());
       ctx.ubo_ctx.add_uniform(&self.name, &mut ctx.resources);
    }

    fn deactive(&mut self,ctx:&mut RenderContext) {
        ctx.ubo_ctx.remove_uniform(&self.name);
    }
}