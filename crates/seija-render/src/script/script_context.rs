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


fn def_ubo(rt:&mut EvalRT,mut args:Vec<Variable>) -> Variable {
    if let Some(var) = rt.find_symbol("UBO_SET").and_then(|v| v.cast_userdata()) {
        let ubo_set = unsafe { &mut *(var as *mut UBOInfoSet)  };
        let json:Value = args.remove(0).into();
        match UBOInfo::try_from(&json) {
            Ok(ubo_info) =>  ubo_set.add_info(ubo_info),
            Err(err) => {
                log::error!("def-ubo error:{:?}",err);
            }
        }
    } else {
        log::error!("def ubo not found UBO_SET");
    }
    Variable::Nil
}

fn node(rt:&mut EvalRT,mut args:Vec<Variable>) -> Variable {
    if args.len() < 2 {
        log::error!("node args < 2");
        return Variable::Nil;
    }
    let node_index = args[0].cast_int().unwrap_or(-1);
    if node_index == -1 {
        log::error!("node args 2 need int");
        return Variable::Nil
    }
    let node_params = args.remove(1);

    let node_creator = rt.find_symbol("NODE_CREATORS").and_then(|v| v.cast_userdata());
    if node_creator.is_none() {
        log::error!("not found NODE_CREATORS");
        return Variable::Nil
    }
    let creators = unsafe { &mut *(node_creator.unwrap() as *mut NodeCreatorContext)  };

    let graph_ctx_ptr = rt.find_symbol("GRAPH_CTX").and_then(|v| v.cast_userdata());
    if graph_ctx_ptr.is_none() {
        log::error!("not found GRAPH_CTX");
        return Variable::Nil
    }
    let graph_ctx = unsafe { &mut *(graph_ctx_ptr.unwrap() as *mut RenderGraphContext)  };
    

    if let Some(node_id) = creators.create(node_index as usize, node_params,graph_ctx) {
        let id_string = node_id.uuid().to_string();
        return id_string.into()
    } else {
        log::error!("not found node_index:{}",node_index);
    }
    
   
    Variable::Nil
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
    (def camera-node (node CAMERA {:ubo "CameraBuffer"}))
    (println camera-node)
    "#,&mut info_set,&mut RenderGraphContext::default());
}