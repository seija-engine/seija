use super::node::{Edge, NodeId};

use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum RenderGraphError {
    #[error("node does not exist")]
    InvalidNodeId(NodeId),
    #[error("node name does not exist")]
    InvalidNodeName(String),
    #[error("attempted to add an edge that already exists")]
    EdgeAlreadyExists(Edge),
}