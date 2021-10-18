use std::{borrow::Cow};
use super::RenderGraphError;
use crate::{render::{RenderContext}, resource::RenderResourceId};
use bevy_ecs::prelude::*;
use uuid::Uuid;


#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NodeId(Uuid);

impl NodeId {
    pub fn new() -> Self { NodeId(Uuid::new_v4()) }
    pub fn uuid(&self) -> &Uuid { &self.0 }
}

pub trait INode: Send + Sync + 'static {
    fn input_count(&self)  -> usize { 0 }
    fn output_count(&self) -> usize { 0 }

    fn prepare(&mut self, _world: &mut World) {}

    fn update(&mut self,world: &mut World,render_ctx:&mut RenderContext,inputs:&Vec<Option<RenderResourceId>>,outputs:&mut Vec<Option<RenderResourceId>>);
}



#[derive(Clone, Debug, Eq, PartialEq)]
pub struct  Edge {
    pub input_node: NodeId,
    pub output_node: NodeId,
    pub input_idxs:Vec<usize>,
    pub output_idxs:Vec<usize>,
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
    //
    pub inputs:Vec<Option<RenderResourceId>>,
    //
    pub outputs:Vec<Option<RenderResourceId>>,
}

impl GraphNode {
    pub fn new<T>(id: NodeId, node: T) -> Self where T: INode {
        let mut inputs:Vec<Option<RenderResourceId>> = Vec::new();
        let mut outputs:Vec<Option<RenderResourceId>> = Vec::new();
        inputs.resize(node.input_count(), None);
        outputs.resize(node.output_count(), None);
        GraphNode {
            id,
            name:None,
            node:Box::new(node),
            inputs,
            outputs,
            edges:Edges {
                node_id:id,
                input_edges: Vec::new(),
                output_edges: Vec::new(),
            }
        }
    }
}


