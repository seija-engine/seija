use std::{convert::{TryInto,TryFrom}, sync::Arc, ptr::NonNull};

use lite_clojure_eval::{EvalRT, ExecScope, Variable};
use anyhow::{Result, Ok};
use serde_json::Value;
use crate::{UniformInfoSet, UniformInfo};

use super::task::{ScriptTask, TaskContext};
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
        self.rt.global_context().push_native_fn("add-uniform", add_uniform);
        self.rt.global_context().push_native_fn("select-add-uniform", select_add_uniform);
    }

    pub fn exec_declare_uniform(&mut self,info_set:&mut UniformInfoSet) {
        let info_ptr = info_set as *mut UniformInfoSet as *mut u8;
        if let Err(err) = self.rt.invoke_func("declare-uniforms", vec![Variable::UserData(info_ptr)]) {
            log::error!("{:?}",err);
        }
    }

    pub fn exec_render_start(&mut self) ->TaskContext {
        let mut task_ctx:TaskContext = TaskContext::new();
        let task_ctx_mut = &mut task_ctx;
        let ptr = (task_ctx_mut as *mut TaskContext) as *mut u8;
        if let Err(err) = self.rt.invoke_func("on-render-start", vec![Variable::UserData(ptr),Variable::Nil]) {
            log::error!("{:?}",err);
        }
        task_ctx
    }

}


fn declare_uniform(scope:&mut ExecScope,args:Vec<Variable>) -> Variable { 
    if _declare_uniform(scope,args).is_none() {  log::error!("declare uniform error"); }
    Variable::Nil
}

fn _declare_uniform(_scope:&mut ExecScope,mut args:Vec<Variable>) -> Option<()> {
    if args.len() != 3 { return None; }
    let info_set = args[0].cast_userdata()?;
    let info_set_mut = unsafe { &mut *(info_set as *mut UniformInfoSet) };
    let name:String = args[1].cast_string()?.borrow().clone();
    let json_value:Value = args.remove(2).into();
    let mut info = UniformInfo::try_from(&json_value).ok()?;
    info.name = Arc::new(name);
    info_set_mut.add_info(info);
    Some(())
}

fn add_uniform(scope:&mut ExecScope,args:Vec<Variable>) -> Variable { 
    if _add_uniform(scope, args).is_none() {
        log::error!("add uniform error");
    }
    Variable::Nil
}

fn _add_uniform(_scope:&mut ExecScope,args:Vec<Variable>) -> Option<()> { 
    if args.len() != 2 { return None; }
    let task_ctx = args[0].cast_userdata()?;
    let task_ctx_mut = unsafe { &mut *(task_ctx as *mut TaskContext) };
    let name:String = args[1].cast_string()?.borrow().clone();
    task_ctx_mut.add_task(ScriptTask::AddUniform(name));
    Some(())
}


fn select_add_uniform(scope:&mut ExecScope,args:Vec<Variable>) -> Variable { 
    if _select_add_uniform(scope, args).is_none() {
        log::error!("add uniform error");
    }
    Variable::Nil
}

fn _select_add_uniform(_scope:&mut ExecScope,args:Vec<Variable>) -> Option<()> { 
    if args.len() != 3 { return None; }
    let task_ctx = args[0].cast_userdata()?;
    let task_ctx_mut = unsafe { &mut *(task_ctx as *mut TaskContext) };
    let tag:String = args[1].cast_string()?.borrow().clone();
    let name:String = args[2].cast_string()?.borrow().clone();
    task_ctx_mut.add_task(ScriptTask::SelectAddUniform(tag,name));
    Some(())
}

#[test]
fn test_declare_uniforms() {
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
    dbg!(info_set);
}

#[test]
fn test_start() {
    env_logger::init();
    let mut sc = ScriptContext::new();
    sc.init(r#"
    (defn on-render-start [this globalEnv]
        (add-uniform this "ObjectBuffer")
        (add-uniform this "CameraBuffer")
        (select-add-uniform this "PBR"    "LightBuffer")
    )
    "#);
    let task_ctx = sc.exec_render_start();
    dbg!(&task_ctx.list);
}