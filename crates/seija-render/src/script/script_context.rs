use std::{convert::{TryInto, TryFrom}, f32::consts::E};

use lite_clojure_eval::{EvalRT, Variable};
use serde_json::Value;

use crate::{UBOInfoSet, UBOInfo};


pub struct RenderScriptContext {
    rt:EvalRT
}

impl RenderScriptContext {
    pub fn new() -> Self {
        let mut rt = EvalRT::new();
        rt.init();
        let mut ctx = RenderScriptContext { rt };
        ctx.add_fns();
        ctx
    }

    fn add_fns(&mut self) {
       self.rt.push_native_fn("def-ubo", def_ubo);
    }

    pub fn run(&mut self,code:&str,info:&mut UBOInfoSet) {
       let info_ptr = info as *mut UBOInfoSet as *mut u8;

       self.rt.push_var("UBO_SET", Variable::UserData(info_ptr));

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


#[test]
fn test_fn() {
    let mut ctx = RenderScriptContext::new();
    let mut info_set = UBOInfoSet::default();
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
    "#,&mut info_set);
}