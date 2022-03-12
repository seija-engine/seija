use std::{convert::{TryInto, TryFrom}, f32::consts::E};

use lite_clojure_eval::{EvalRT, Variable};
use serde_json::Value;

use crate::{UBOInfoSet, UBOInfo, graph::{NodeId, self}, render::RenderGraphContext};

use super::{NodeCreatorContext, NodeCreatorSet, NodeCreatorFn};


pub struct RenderScriptContext {
    rt:EvalRT,
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
       self.rt.push_native_fn("def-ubo", def_ubo);
       self.rt.push_native_fn("node", node);
       self.rt.push_native_fn("link->", link_node);
       
    }

    pub fn add_node_creator(&mut self,name:&str,f:NodeCreatorFn) {
        let index = self.node_creators.add_creator(f);
        self.rt.push_var(name, Variable::Int(index as i64));
    }

    pub fn add_node_creator_set(&mut self,set:&NodeCreatorSet) {
        for (k,v) in set.0.iter() {
            self.add_node_creator(k, *v);
        }
    }

    pub fn run(&mut self,code:&str,info:&mut UBOInfoSet,graph_ctx:&mut RenderGraphContext) {
       let info_ptr = info as *mut UBOInfoSet as *mut u8;
       self.rt.push_var("UBO_SET", Variable::UserData(info_ptr));
       
       let creator_ptr = (&mut self.node_creators) as *mut NodeCreatorContext as *mut u8;
       self.rt.push_var("NODE_CREATORS", Variable::UserData(creator_ptr));

       let graph_ptr = graph_ctx as *mut RenderGraphContext as *mut u8;
       self.rt.push_var("GRAPH_CTX", Variable::UserData(graph_ptr));

       self.rt.eval_string("render".into(), code);
    }
}

fn find_userdata<'a,T>(rt:&'a EvalRT,name:&str) -> Option<&'a mut T> {
    let ptr = rt.find_symbol(name).and_then(|v| v.cast_userdata());
    if ptr.is_none() {
        log::error!("not found {}",name);
        return None;
    }
    Some(unsafe { &mut *(ptr.unwrap() as *mut T)  })
}


fn def_ubo(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    (|rt:&mut EvalRT,mut args:Vec<Variable>| {
        let info_set = find_userdata::<UBOInfoSet>(rt, "UBO_SET")?;
        let json:Value =  args.remove(0).into();
        match UBOInfo::try_from(&json) {
            Ok(ubo_info) =>  info_set.add_info(ubo_info),
            Err(err) => {
                log::error!("def-ubo error:{:?}",err);
            }
        }
        Some(Variable::Nil)
    })(rt,args).unwrap_or(Variable::Nil)
}

fn node(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() < 1 {
        log::error!("node args < 2");
        return Variable::Nil;
    }
    (|rt:&mut EvalRT,mut args:Vec<Variable>| {
        let node_index = args[0].cast_int()?;
        let node_params = if args.len() > 1 { args.remove(1) } else {Variable::Nil };
        let node_creator = find_userdata::<NodeCreatorContext>(rt, "NODE_CREATORS")?;
        let graph_ctx = find_userdata::<RenderGraphContext>(rt, "GRAPH_CTX")?;
        let node_id = node_creator.create(node_index as usize, node_params,graph_ctx)?;
        Some(node_id.uuid().to_string().into())
    })(rt,args).unwrap_or(Variable::Nil)
}


fn link_node(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        log::error!("node args < 3");
        return Variable::Nil;
    }
    (|rt:&mut EvalRT,args:Vec<Variable>| {
        let graph_ctx = find_userdata::<RenderGraphContext>(rt, "GRAPH_CTX")?;
        let snode_1 = args[0].cast_string()?;
        let snode_2 = args[1].cast_string()?;
        let node_1 = NodeId::try_from(snode_1.borrow().as_str()).ok()?;
        let node_2 = NodeId::try_from(snode_2.borrow().as_str()).ok()?;
        
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
    let mut info_set = UBOInfoSet::default();
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
    "#,&mut info_set,&mut RenderGraphContext::default());
}