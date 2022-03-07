use std::collections::HashMap;

use lite_clojure_eval::Variable;
use crate::{graph::NodeId, render::RenderGraphContext};

pub type NodeCreatorFn = fn(ctx:&mut RenderGraphContext,Variable) -> NodeId;

#[derive(Default)]
pub struct NodeCreatorContext {
    creators:Vec<NodeCreatorFn>
}

impl NodeCreatorContext {
    pub fn add_creator(&mut self,f:NodeCreatorFn) -> u32 {
        self.creators.push(f);
        (self.creators.len() - 1) as u32
    }

    pub fn create(&self,index:usize,var:Variable,ctx:&mut RenderGraphContext) -> Option<NodeId> {
        if let Some(f) = self.creators.get(index) {
           Some((*f)(ctx,var))
        } else {
            None
        }
    }
}

pub struct NodeCreatorSet(pub HashMap<String,NodeCreatorFn>);