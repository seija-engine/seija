use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::World;
use lite_clojure_eval::{Variable, EvalRT};

use crate::RenderContext;

use super::{main::MainContext, rt_tags::RuntimeTags};

pub trait IUpdateNode {
    fn update_params(&mut self,params:Vec<Variable>);

    fn init(&mut self,world:&mut World,ctx:&mut RenderContext);

    fn prepare(&mut self,world:&mut World,ctx:&mut RenderContext) {}

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {}
}

pub struct UpdateNodeBox {
    pub tag_index:Option<usize>,
    pub enable:bool,
    pub params:Vec<Variable>,
    pub node:Box<dyn IUpdateNode>
}

impl UpdateNodeBox {
    pub fn create<T>(params:&Vec<Variable>) -> UpdateNodeBox where T:Default + IUpdateNode + 'static {
        let node:T = Default::default();
        UpdateNodeBox { tag_index:None,enable:true, params:params.clone(), node:Box::new(node) }
    }

    pub fn set_params(&mut self,rt:&mut EvalRT,is_first:bool) {
        if !is_first {
            if !self.params.iter().any(|v| match v {
                Variable::Function(_) => true,
                _ => false
            }) { return; }
        }

        let mut new_params:Vec<Variable> = vec![];
        for param in self.params.iter() {
            match param {
                Variable::Function(_) => { 
                    match rt.invoke_func2(param, vec![]) {
                        Ok(v) => { new_params.push(v); },
                        Err(err) => { log::error!("{:?}",err); }
                    }
                 },
                _ => {
                    new_params.push(param.clone());
                }
            }
        }
        self.node.update_params(new_params);
    }

    pub fn init(&mut self,world:&mut World,ctx:&mut RenderContext) {
        self.node.init(world, ctx);
    }
    
    pub fn update_enable(&mut self,rt_tags:&RuntimeTags) {
        if let Some(tag_index) = self.tag_index {
            if rt_tags.dirtys.contains(&tag_index) && rt_tags.tags[tag_index] != self.enable { 
                self.enable = !self.enable;
            }
        }
    }

    pub fn prepare(&mut self,world:&mut World,ctx:&mut RenderContext) {
        self.node.prepare(world, ctx);
    }

    pub fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
        if self.enable {
            self.node.update(world, ctx);
        }
    }
}

pub type NodeCreatorFn = fn(ctx:&mut MainContext,Vec<Variable>) -> UpdateNodeBox;

#[derive(Default)]
pub struct NodeCreatorSet(pub HashMap<String,NodeCreatorFn>);

impl NodeCreatorSet {
    pub fn add<T>(&mut self,name:&str) where T:Default + IUpdateNode + 'static {
        self.0.insert(name.to_string(), |_,args| UpdateNodeBox::create::<T>(&args) );
    }
}

#[derive(Default)]
pub struct NodeCreatorContext {
   pub creators:Vec<NodeCreatorFn>
}

impl NodeCreatorContext {
    pub fn add(&mut self,f:NodeCreatorFn) -> usize {
        self.creators.push(f);
        self.creators.len() - 1
    }
}