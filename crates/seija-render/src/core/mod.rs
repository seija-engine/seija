mod render_context;
pub mod resource;

pub use render_context::{IRenderResourceContext};

pub trait IBackend {
    fn resource_context(&self) -> &dyn IRenderResourceContext;
}