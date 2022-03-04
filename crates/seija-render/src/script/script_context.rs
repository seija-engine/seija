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
       self.rt.push_native_fn("def-ubo", def_ubo);
    }

    pub fn run(&mut self,code:&str) {
       self.rt.eval_string("render".into(), code);
    }
}



fn def_ubo(rt:&mut EvalRT,args:Vec<Variable>) -> Variable {
    
    Variable::Nil
}


#[test]
fn test_fn() {
    let mut ctx = RenderScriptContext::new();
    ctx.run(r#"
    (add-backend 123)
    
    "#);

}