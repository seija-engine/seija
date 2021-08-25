use std::borrow::Cow;
use super::RenderGraphError;
use uuid::Uuid;


#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NodeId(Uuid);

impl NodeId {
    pub fn new() -> Self { NodeId(Uuid::new_v4()) }
    pub fn uuid(&self) -> &Uuid { &self.0 }
}

pub trait INode: Send + Sync + 'static {

}



#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Edge {
    SlotEdge {
        input_node: NodeId,
        input_index: usize,
        output_node: NodeId,
        output_index: usize,
    },
    NodeEdge {
        input_node: NodeId,
        output_node: NodeId,
    },
}

#[derive(Debug)]
pub struct Edges {
    pub node_id: NodeId,
    pub input_edges: Vec<Edge>,
    pub output_edges: Vec<Edge>,
}

impl Edges {
    pub fn has_input_edge(&self, edge: &Edge) -> bool {
        self.input_edges.contains(edge)
    }

    pub fn has_output_edge(&self, edge: &Edge) -> bool {
        self.output_edges.contains(edge)
    }

    pub(crate) fn add_input_edge(&mut self, edge: Edge) -> Result<(), RenderGraphError> {
        if self.has_input_edge(&edge) {
            return Err(RenderGraphError::EdgeAlreadyExists(edge));
        }
        self.input_edges.push(edge);
        Ok(())
    }

    pub fn add_output_edge(&mut self, edge: Edge) -> Result<(), RenderGraphError> {
        if self.has_output_edge(&edge) {
           return Err(RenderGraphError::EdgeAlreadyExists(edge));
        }
        self.output_edges.push(edge);
        Ok(())
    }
}

pub struct GraphNode {
    pub id:NodeId,
    pub name:Option<Cow<'static,str>>,
    pub node:Box<dyn INode>,
    pub edges: Edges,
}

impl GraphNode {
    pub fn new<T>(id: NodeId, node: T) -> Self where T: INode {
        GraphNode {
            id,
            name:None,
            node:Box::new(node),
            edges:Edges {
                node_id:id,
                input_edges: Vec::new(),
                output_edges: Vec::new(),
            }
        }
    }
}