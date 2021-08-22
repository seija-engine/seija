use super::resource::{self, buffer::{BufferId, BufferInfo}};

pub trait IRenderResourceContext: Send + Sync + 'static {
    fn create_swap_chain(&self);
    
    fn create_buffer(&self, buffer_info: BufferInfo) -> BufferId;
}

pub trait IRenderContext {
    fn resources(&self) -> &dyn IRenderResourceContext;
    fn resources_mut(&mut self) -> &mut dyn IRenderResourceContext;
}