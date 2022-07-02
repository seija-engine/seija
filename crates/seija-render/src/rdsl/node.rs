use bevy_ecs::prelude::World;
use lite_clojure_eval::{Variable, EvalRT};

use crate::RenderContext;

pub trait IUpdateNode {
    fn update_params(&mut self,params:Vec<Variable>);

    fn init(&mut self,world:&mut World,ctx:&mut RenderContext);

    fn prepare(&mut self,world:&mut World,ctx:&mut RenderContext) {}

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {}
}

pub struct UpdateNodeBox {
    pub params:Vec<Variable>,
    pub node:Box<dyn IUpdateNode>
}

impl UpdateNodeBox {
    pub fn create<T>(params:&Vec<Variable>) -> UpdateNodeBox where T:Default + IUpdateNode + 'static {
        let node:T = Default::default();
        UpdateNodeBox { params:params.clone(), node:Box::new(node) }
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

    pub fn prepare(&mut self,world:&mut World,ctx:&mut RenderContext) {
        self.node.prepare(world, ctx);
    }

    pub fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
        self.node.update(world, ctx);
    }
   
}