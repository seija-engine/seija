use std::collections::{HashSet, VecDeque};
use std::{borrow::Cow, collections::HashMap, fmt::Debug};
use bevy_ecs::prelude::World;
use seija_core::IDGenU32;

use crate::RenderContext;

use super::node::{Edge, GraphNode, INode, NodeId};
use super::RenderGraphError;

pub struct RenderGraph {
    id_gen:IDGenU32,
    nodes: HashMap<NodeId, GraphNode>,
    node_names: HashMap<Cow<'static, str>, NodeId>,
    pub iter:LinearGraphIter,
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
            id_gen:IDGenU32::new(),
            nodes:Default::default(),
            node_names: Default::default(),
            iter:LinearGraphIter::default(),
        }
    }
}

impl RenderGraph {
    pub fn iter_nodes(&self) -> impl Iterator<Item = &GraphNode> {
        self.nodes.values()
    }
    pub fn iter_mut_nodes(&mut self) -> impl Iterator<Item = &mut GraphNode> {
        self.nodes.values_mut()
    }

    pub fn add_node<T>(&mut self,name:impl Into<Cow<'static, str>>,node:T) -> NodeId where T:INode {
        let id = NodeId(self.id_gen.next());
        let name = name.into();
        let mut graph_node = GraphNode::new(id, node);
        graph_node.name = Some(name.clone());
        self.nodes.insert(id, graph_node);
        self.node_names.insert(name, id);
        id
    }

    pub fn get_node(&self,id:&NodeId) -> Result<&GraphNode,RenderGraphError> {
        self.nodes.get(id).ok_or(RenderGraphError::InvalidNodeId(id.clone()))
    }

    pub fn get_node_mut(&mut self,id:&NodeId) -> Result<&mut GraphNode,RenderGraphError> {
        self.nodes.get_mut(id).ok_or(RenderGraphError::InvalidNodeId(id.clone()))
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

    pub fn add_link(&mut self,from:NodeId,to:NodeId,from_idxs:Vec<usize>,to_idxs:Vec<usize>) -> Result<(),RenderGraphError> {
        let edge = Edge {
            output_node: from,
            input_node: to,
            input_idxs: to_idxs,
            output_idxs:from_idxs,
        };
       
        let from_node = self.get_node_mut(&from)?;
        from_node.edges.add_output_edge(edge.clone())?;

        let to_node = self.get_node_mut(&to)?;
        to_node.edges.add_input_edge(edge)?;
        Ok(())
    }

    pub fn build_iter(&mut self) {
        let line_graph = LinearGraphIter::from_graph(self);
        self.iter = line_graph;
    }

    pub fn prepare(&mut self,world:&mut World,ctx:&mut RenderContext) {
        for node in self.nodes.values_mut() {
            node.node.prepare(world,ctx);
        }

       
    }
}

#[derive(Default)]
pub struct LinearGraphIter {
    pub nodes:Vec<NodeId>
}

//
// A -> B -> C -> D
//    /     /
//  E      F -> G
//
// H -> I -> J
// 1. A E F H
// 2. B G C
// D

// A E F H B G C D
impl LinearGraphIter {
    pub fn from_graph(graph:&RenderGraph) -> LinearGraphIter {
        let only_outputs = graph.iter_nodes().filter(|n| n.edges.input_edges.is_empty());
        let mut queue:VecDeque<NodeId> = VecDeque::new();
        let mut ret_list:Vec<NodeId> = Vec::new();
        let mut dic:HashSet<NodeId> = HashSet::new();
        for node in only_outputs {
            queue.push_back(node.id);
            ret_list.push(node.id);
            dic.insert(node.id);
        }
        while !queue.is_empty() {
            let first = queue.pop_front().unwrap();
            let first_node = graph.nodes.get(&first).unwrap();
            if !dic.contains(&first) {
                if Self::is_parent_in_dic(&dic, &first_node) {
                    ret_list.push(first);
                    dic.insert(first);
                } else {
                    queue.push_back(first);
                }
            }

            for out_node_edge in first_node.edges.output_edges.iter() {
                let out_node_id = &out_node_edge.input_node;
                if dic.contains(out_node_id) { continue };
                queue.push_back(*out_node_id);
                let out_node = graph.get_node(out_node_id).unwrap();
                if Self::is_parent_in_dic(&dic, out_node) {
                    ret_list.push(*out_node_id);
                    dic.insert(*out_node_id);
                }
            }
        }
        LinearGraphIter {
            nodes:ret_list
        }
    }

    fn is_parent_in_dic(dic:&HashSet<NodeId>,node:&GraphNode) -> bool {
        for input_edge in node.edges.input_edges.iter() {
            if !dic.contains(&input_edge.output_node) {
                return false;
            }
        }
        true
    }
}


mod test {
    use crate::{RenderContext, graph::{ node::INode}};
    use bevy_ecs::prelude::*;
    use crate::resource::RenderResourceId;

   
    struct  TestNode;
    impl INode for TestNode {
        fn update(&mut self,_world: &mut World,_render_ctx:&mut RenderContext,_inputs:&Vec<Option<RenderResourceId>>,_outputs:&mut Vec<Option<RenderResourceId>>) {}
    }

    #[test]
    fn test_graph() {
        use super::LinearGraphIter;
        use crate::graph::{RenderGraph};
        let mut graph = RenderGraph::default();
        let a_id = graph.add_node("node_a", TestNode);
        let b_id = graph.add_node("node_b", TestNode);
        let c_id = graph.add_node("node_c", TestNode);

        let d_id = graph.add_node("node_d", TestNode);
        let e_id = graph.add_node("node_e", TestNode);
        let f_id = graph.add_node("node_f", TestNode);

        graph.add_link(a_id, b_id,vec![],vec![]).unwrap();
        graph.add_link(b_id, c_id,vec![],vec![]).unwrap();
        
        graph.add_link(d_id, a_id,vec![],vec![]).unwrap();
        graph.add_link(d_id, e_id,vec![],vec![]).unwrap();
        graph.add_link(e_id, b_id,vec![],vec![]).unwrap();
        graph.add_link(f_id, c_id,vec![],vec![]).unwrap();
        
        let line_graph = LinearGraphIter::from_graph(&graph);
        for node_id in line_graph.nodes {
            let node = graph.get_node(&node_id).unwrap();
            print!("{:?} -> ",node.name.as_ref().unwrap());
        }
    }

}