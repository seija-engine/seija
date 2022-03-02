

use std::{ffi::c_void, ptr::NonNull};

use lite_clojure_eval::{EvalRT, Variable};
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
       self.rt.push_native_fn("add-backend", add_backend);
    }

    pub fn run(&mut self,code:&str) {
        let ptr:*mut c_void = (self as *mut RenderScriptContext) as *mut c_void;
        
       self.rt.eval_string("render".into(), code);
       
    }
}

fn add_backend(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    
    Variable::Nil
}


#[test]
fn test_fn() {
    let mut ctx = RenderScriptContext::new();
    ctx.run(r#"
    (add-backend 123)
    
    "#);

}