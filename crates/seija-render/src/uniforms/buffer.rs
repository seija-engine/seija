use crate::{memory::TypedUniformBuffer, resource::{BufferId, RenderResources}, UBOInfo};


pub struct UBObject {
    local:TypedUniformBuffer,
    cache:Option<BufferId>,
    buffer:BufferId
}

impl UBObject {
    pub fn create(info:&UBOInfo,res:&mut RenderResources) -> Self {
        let local = TypedUniformBuffer::from_def(info.props.clone());
        let buffer = res.create_buffer(&wgpu::BufferDescriptor {
            label:None,
            size:info.props.size() as u64,
            usage:wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM,
            mapped_at_creation:false
        });
        UBObject {local, cache:None,buffer }
    }
}