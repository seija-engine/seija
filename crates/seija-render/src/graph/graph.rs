use std::{borrow::Cow, collections::HashMap, fmt::Debug};
use super::node::{Edge, GraphNode, INode, NodeId};
use super::RenderGraphError;

pub struct RenderGraph {
    nodes: HashMap<NodeId, GraphNode>,
    node_names: HashMap<Cow<'static, str>, NodeId>
}

impl Debug for RenderGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in self.iter_nodes() {
            writeln!(f, "{:?}", node.id)?;
            writeln!(f, "  edges: {:?}", node.edges)?;
        }

        Ok(())
    }
}

impl Default for RenderGraph {
    fn default() -> Self {
        Self {
            nodes:Default::default(),
            node_names: Default::default(),
        }
    }
}

impl RenderGraph {

    pub fn iter_nodes(&self) -> impl Iterator<Item = &GraphNode> {
        self.nodes.values()
    }

    pub fn add_node<T>(&mut self,name:impl Into<Cow<'static, str>>,node:T) -> NodeId where T:INode {
        let id = NodeId::new();
        let name = name.into();
        let mut graph_node = GraphNode::new(id, node);
        graph_node.name = Some(name.clone());
        self.nodes.insert(id, graph_node);
        self.node_names.insert(name, id);
        id
    }

    pub fn get_node(&self,id:NodeId) -> Result<&GraphNode,RenderGraphError> {
        self.nodes.get(&id).ok_or(RenderGraphError::InvalidNodeId(id))
    }

    pub fn get_node_mut(&mut self,id:NodeId) -> Result<&mut GraphNode,RenderGraphError> {
        self.nodes.get_mut(&id).ok_or(RenderGraphError::InvalidNodeId(id))
    }

    pub fn get_node_id(&self, name: &str) -> Result<NodeId, RenderGraphError> {
        self.node_names.get(name).map(|v| v.clone()).ok_or(RenderGraphError::InvalidNodeName(name.to_owned()))
    }

    pub fn get_node_by_name(&self,name:&str) -> Result<&GraphNode,RenderGraphError> {
        self.node_names.get(name).and_then(|v| self.nodes.get(v)).ok_or(RenderGraphError::InvalidNodeName(name.to_owned()))
    }

    pub fn get_node_by_name_mut(&mut self,name:&str) -> Result<&mut GraphNode,RenderGraphError> {
        if let Some(node_id) = self.node_names.get(name) {
            self.nodes.get_mut(node_id).ok_or(RenderGraphError::InvalidNodeName(name.to_owned()))
        } else {
            Err(RenderGraphError::InvalidNodeName(name.to_owned()))
        }
    }

    pub fn add_link(&mut self,from:NodeId,to:NodeId) -> Result<(),RenderGraphError> {
        let edge = Edge::NodeEdge {
            output_node: from,
            input_node: to,
        };
        let from_node = self.get_node_mut(from)?;
        from_node.edges.add_output_edge(edge.clone())?;

        let to_node = self.get_node_mut(to)?;
        to_node.edges.add_input_edge(edge)?;
        Ok(())
    }
}

mod test {
    use crate::graph::{graph::RenderGraph, node::INode};

    struct  TestNode;
    impl INode for TestNode {}

    #[test]
    fn test_graph() {
        let mut graph = RenderGraph::default();
        let a_id = graph.add_node("node_a", TestNode);
        let b_id = graph.add_node("node_b", TestNode);
        graph.add_link(a_id, b_id).unwrap();

        dbg!(graph);
    }

}