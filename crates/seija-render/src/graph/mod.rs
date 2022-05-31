mod node;
mod error;
mod graph;
pub mod nodes;
pub use node::{NodeId,INode};
pub use error::RenderGraphError;
pub use graph::{RenderGraph,LinearGraphIter};
use thiserror::{Error};
#[derive(Debug,Error)]
pub enum GraphError {
    #[error("error input {0}")]
    ErrInput(usize),
    #[error("error target view")]
    ErrTargetView,
    #[error("miss mesh")]
    MissMesh,
    #[error("miss material")]
    MissMaterial,
    #[error("error ubo index")]
    ErrUBOIndex
}