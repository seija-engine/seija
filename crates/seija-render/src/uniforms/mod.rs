use crate::resource::BufferId;
use std::collections::{HashMap};

pub struct GPUUniform {
    buffer:Option<BufferId>,
    layout:wgpu::BindGroupLayout,
    bind_group:Option<wgpu::BindGroup>
}

#[derive(Default)]
pub struct GPUUniformList {
  list:HashMap<String,GPUUniform>
}

impl GPUUniformList {

}


