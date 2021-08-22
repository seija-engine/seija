use std::{collections::HashMap, sync::{Arc}};
use parking_lot::RwLock;
use seija_render::core::{IRenderResourceContext,resource::buffer::{BufferInfo,BufferId}};

use crate::type_converter::WgpuInto;


#[derive(Default, Clone, Debug)]
pub struct Resources {
    pub buffer_infos: Arc<RwLock<HashMap<BufferId, BufferInfo>>>,
    pub buffers: Arc<RwLock<HashMap<BufferId, Arc<wgpu::Buffer>>>>,
}

#[derive(Clone, Debug)]
pub struct RenderResourceContext {
    pub device: Arc<wgpu::Device>,
    pub resources:Resources
}

impl RenderResourceContext {
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        RenderResourceContext {
            device,
            resources: Resources::default(),
        }
    }

}

impl IRenderResourceContext for RenderResourceContext {
    fn create_swap_chain(&self) {
        todo!()
    }

    fn create_buffer(&self, info: BufferInfo) -> BufferId {
        let mut buffer_infos = self.resources.buffer_infos.write();
        let mut buffers = self.resources.buffers.write();
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: info.size as u64,
            usage: info.buffer_usage.wgpu_into(),
            mapped_at_creation: info.mapped_at_creation,
        });

        let id = BufferId::new();
        buffer_infos.insert(id, info);
        buffers.insert(id, Arc::new(buffer));
        id
    }
}
