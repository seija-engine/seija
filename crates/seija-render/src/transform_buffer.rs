use bevy_ecs::prelude::World;
use wgpu::Device;

use crate::resource::{BufferId, RenderResources};

pub struct TransformBuffer {
    cur_size:usize,
    stage_buffer:Option<BufferId>
}

impl TransformBuffer {
    pub fn new() -> TransformBuffer {
        TransformBuffer {
            cur_size : 0,
            stage_buffer:None
        }
    }

    pub fn update(&self,world:&mut World) {

    }
}