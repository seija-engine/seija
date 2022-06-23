use std::{convert::{TryFrom}};

use lite_clojure_eval::{EvalRT, Variable, ExecScope};
use serde_json::Value;

use crate::{UniformInfoSet, UniformInfo, graph::{NodeId}, render::RenderGraphContext};

use super::{NodeCreatorContext, NodeCreatorSet, NodeCreatorFn,RenderScriptPlugin};


pub struct RenderScriptContext {
    pub rt:EvalRT,
    node_creators:NodeCreatorContext
}

impl RenderScriptContext {
    pub fn new() -> Self {
        let mut rt = EvalRT::new();
        rt.init();
        let mut ctx = RenderScriptContext { rt,node_creators:NodeCreatorContext::default() };
        ctx.add_fns();
        ctx
    }

    fn add_fns(&mut self) {
       self.rt.global_context().push_var("SS_VERTEX", wgpu::ShaderStage::VERTEX.bits() as i64 );
       self.rt.global_context().push_var("SS_FRAGMENT", wgpu::ShaderStage::FRAGMENT.bits() as i64 );
       self.rt.global_context().push_var("SS_VERTEX_FRAGMENT", wgpu::ShaderStage::VERTEX_FRAGMENT.bits() as i64 );
       self.rt.global_context().push_var("SS_COMPUTE", wgpu::ShaderStage::COMPUTE.bits() as i64 );
       self.rt.global_context().push_var("SS_ALL", wgpu::ShaderStage::all().bits() as i64 );

       self.rt.global_context().push_native_fn("add-uniform", def_uniform);
       self.rt.global_context().push_native_fn("node", node);
       self.rt.global_context().push_native_fn("link->", link_node);
       
    }

    pub fn add_node_creator(&mut self,name:&str,f:NodeCreatorFn) {
        let index = self.node_creators.add_creator(f);
        self.rt.global_context().push_var(name, Variable::Int(index as i64));
    }

    pub fn add_node_creator_set(&mut self,set:&NodeCreatorSet) {
        for (k,v) in set.0.iter() {
            self.add_node_creator(k, *v);
        }
    }

    pub fn run(&mut self,code:&str,info:&mut UniformInfoSet,graph_ctx:&mut RenderGraphContext,is_create_graph:bool) {
       let info_ptr = info as *mut UniformInfoSet as *mut u8;
       self.rt.global_context().push_var("UBO_SET", Variable::UserData(info_ptr));
       
       let creator_ptr = (&mut self.node_creators) as *mut NodeCreatorContext as *mut u8;
       self.rt.global_context().push_var("NODE_CREATORS", Variable::UserData(creator_ptr));

       let graph_ptr = graph_ctx as *mut RenderGraphContext as *mut u8;
       self.rt.global_context().push_var("GRAPH_CTX", Variable::UserData(graph_ptr));

       self.rt.eval_string("render".into(), code);
       if is_create_graph {
         if let Err(err) = self.rt.invoke_func("create-graph",vec![]) {
            log::error!("render.clj error:{:?}",err);
         }
       }
    }
}

fn find_userdata<'a,T>(rt:&'a ExecScope,name:&str) -> Option<&'a mut T> {
    let ptr = rt.context.find_symbol(None, name, &rt.modules).and_then(|v| v.cast_userdata());
    if ptr.is_none() {
        log::error!("not found {}",name);
        return None;
    }
    Some(unsafe { &mut *(ptr.unwrap() as *mut T)  })
}


fn def_uniform(rt:&mut ExecScope,args:Vec<Variable>) -> Variable {
    (|rt:&mut ExecScope,mut args:Vec<Variable>| {
        let info_set = find_userdata::<UniformInfoSet>(rt, "UBO_SET")?;
        let json:Value =  args.remove(0).into();
        match UniformInfo::try_from(&json) {
            Ok(ubo_info) =>  info_set.add_info(ubo_info),
            Err(err) => {
                log::error!("def-ubo error:{:?}",err);
            }
        }
        Some(Variable::Nil)
    })(rt,args).unwrap_or(Variable::Nil)
}

fn node(rt:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() < 1 {
        log::error!("node args < 1");
        return Variable::Nil;
    }
    (|rt:&mut ExecScope,mut args:Vec<Variable>| {
        let node_index = args[0].cast_int()?;
        let node_params = if args.len() > 1 { args.remove(1) } else {Variable::Nil };
        let node_creator = find_userdata::<NodeCreatorContext>(rt, "NODE_CREATORS")?;
        let graph_ctx = find_userdata::<RenderGraphContext>(rt, "GRAPH_CTX")?;
        let node_id = node_creator.create(node_index as usize, node_params,graph_ctx)?;
        Some(Variable::Int(node_id.0 as i64))
    })(rt,args).unwrap_or(Variable::Nil)
}


fn link_node(rt:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        log::error!("node args < 3");
        return Variable::Nil;
    }
    (|rt:&mut ExecScope,args:Vec<Variable>| {
        let graph_ctx = find_userdata::<RenderGraphContext>(rt, "GRAPH_CTX")?;
        let snode_1 = args[0].cast_int()?;
        let snode_2 = args[1].cast_int()?;
        let node_1 = NodeId(snode_1 as u32);
        let node_2 = NodeId(snode_2 as u32);
        
        let mut from_idxs:Vec<usize> = vec![];
        let mut to_idxs:Vec<usize>   = vec![];
        if args.len() > 2 {
            let slot_map = args[2].cast_map()?;
            for (k,v) in slot_map.borrow().iter() {
                let from = k.cast_int()? as usize;
                let to = v.cast_int()? as usize;
                from_idxs.push(from);
                to_idxs.push(to);
            }
        }
        if let Err(err) = graph_ctx.graph.add_link(node_1, node_2, from_idxs, to_idxs) {
            log::error!("link-> err:{}",err);
        }
        None
    })(rt,args).unwrap_or(Variable::Nil)
}


#[test]
fn test_fn() {
    env_logger::init();
    let mut ctx = RenderScriptContext::new();
    let mut info_set = UniformInfoSet::default();
    let buildin = crate::builtin_node_creators();
    ctx.add_node_creator_set(&buildin);
    ctx.run(r#"
    (def-ubo {
        :type :PerCamera
        :name "CameraBuffer"
        :backends ["Camera3D"]
        :props [
            {:name "projView" :type "mat4"  }
            {:name "view"     :type "mat4"  }
            {:name "proj"     :type "mat4"  }
            {:name "pos"      :type "float4"}
        ]
    })
    (def camera-node  (node CAMERA {:ubo "CameraBuffer"}))
    (def camera-node2 (node CAMERA {:ubo "CameraBuffer"}))
    (link-> camera-node camera-node2 {0 1})
    "#,&mut info_set,&mut RenderGraphContext::default(),true);
}