use lite_clojure_eval::{EvalRT, ExecScope, Variable};

use crate::UniformInfoSet;
pub struct ScriptContext {
    rt:EvalRT
}

impl ScriptContext {
    pub fn new() -> Self {
        let mut rt = EvalRT::new();
        rt.init();
        ScriptContext {  
            rt
        }
    }

    pub fn init(&mut self,code_string:&str) {
        self.rt.eval_string("render.clj".into(), code_string);
        self.rt.global_context().push_native_fn("declare-uniform", declare_uniform);
    }

    pub fn exec_declare_uniform(&mut self,info_set:&mut UniformInfoSet) {
        let info_ptr = info_set as *mut UniformInfoSet as *mut u8;
        if let Err(err) = self.rt.invoke_func("declare-uniforms", vec![Variable::UserData(info_ptr)]) {
            log::error!("{:?}",err);
        }
    }

}


fn declare_uniform(scope:&mut ExecScope,args:Vec<Variable>) -> Variable { 
    if let Some(info_set) = args[0].cast_userdata().map(|v| v as *mut UniformInfoSet) {
        let info_set_mut = unsafe { &mut *info_set };
        let name = args[1].cast_string();
    }
    Variable::Nil
}

fn _declare_uniform(scope:&mut ExecScope,args:Vec<Variable>) {
    
}

#[test]
fn test_script() {
    env_logger::init();
    let mut sc = ScriptContext::new();
    sc.init(r#"
    (defn declare-uniforms [set]
        (declare-uniform set "ObjectBuffer" {
            :type :Component
            :apply :RenderObject
            :sort 1
            :shader-stage 1
            :props [
               {:name "transform" :type "mat4"}
            ]
            :backends ["Transform"]
        })
    )
   
    "#);
    let mut info_set:UniformInfoSet = UniformInfoSet::default();
    sc.exec_declare_uniform(&mut info_set);
}