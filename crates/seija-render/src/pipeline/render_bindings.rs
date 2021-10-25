use std::num::NonZeroU32;
use crate::resource::RenderResourceId;

pub struct RenderBinding {
    res_id:RenderResourceId
}

#[derive(Default)]
pub struct RenderBindings {
    layout_entrys:Vec<wgpu::BindGroupLayoutEntry>
}

impl RenderBindings {
    pub fn add_binding(&mut self,stage:wgpu::ShaderStage,ty:wgpu::BindingType,count:Option<NonZeroU32>) {
        let layout_entry = wgpu::BindGroupLayoutEntry {
            binding:self.layout_entrys.len() as u32,
            visibility:stage,
            ty,
            count:None
        };
        
        self.layout_entrys.push(layout_entry);
    }
}