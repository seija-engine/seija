use lite_clojure_eval::{EvalRT, ExecScope, Variable, run_native_fn};
use serde_json::Value;
use thiserror::Error;
use anyhow::{anyhow,Result};
use std::{convert::TryFrom, sync::Arc};
use crate::{UniformInfo, UniformInfoSet};

use super::builder::{FRPCompBuilder, BuilderCommand};

#[derive(Debug,Error)]

enum Errors {
    #[error("not found info set")]
    NotFoundInfoSet,
    #[error("type cast error {0}")]
    TypeCastError(&'static str),
    #[error("not found userdata {0}")]
    NotFoundUserData(&'static str)
}

pub fn init_fns(vm:&mut EvalRT) {
    vm.global_context().push_native_fn("declare-uniform", declare_uniform);
    vm.global_context().push_native_fn("__frp_enter__", __frp_enter__);
    vm.global_context().push_native_fn("uniform", uniform);
}

pub fn declare_uniform(s:&mut ExecScope,a:Vec<Variable>) -> Variable { 
    run_native_fn("declare-uniform", s, a, |_,mut args| {
        let info_set = args[0].cast_userdata().ok_or(Errors::NotFoundInfoSet)?;
        let name:String = args[1].cast_string().ok_or(Errors::TypeCastError("string"))?.borrow().clone();
        let json_value:Value = args.remove(2).into();
        let mut info = UniformInfo::try_from(&json_value).map_err(|e| anyhow!(e))?;
        info.name =  Arc::new(name);
        let info_set_mut = unsafe { &mut *(info_set as *mut UniformInfoSet) };
        info_set_mut.add_info(info);
        Ok(Variable::Nil)
    })
}

fn find_userdata<T>(scope:&mut ExecScope,name:&'static str) -> Result<&'static mut T> {
    let textures = scope.context.find_symbol(None, name, &scope.modules)
                                                          .ok_or(Errors::NotFoundUserData(name))?;
    let ptr = textures.cast_userdata().ok_or(Errors::TypeCastError("userdata"))?;
    let value_ptr = unsafe { &mut *(ptr as *mut T) };
    Ok(value_ptr)
}

fn find_frp_builder(scope:&mut ExecScope) -> Result<&'static mut FRPCompBuilder> {
    find_userdata::<FRPCompBuilder>(scope, "*BUILDER*")
}

pub fn __frp_enter__(s:&mut ExecScope,a:Vec<Variable>) -> Variable {
    run_native_fn("__frp_enter__", s, a, |scope,args| {
        let name:String = args[0].cast_string().ok_or(Errors::TypeCastError("string"))?.borrow().clone();
        let command = BuilderCommand::StartComp(name);
        let builder = find_frp_builder(scope)?;
        builder.push_command(command);
        Ok(Variable::Nil)
    })
}

pub fn uniform(s:&mut ExecScope,a:Vec<Variable>) -> Variable { 
    run_native_fn("uniform", s, a, |scope,args| {
        let name:String = args[0].cast_string().ok_or(Errors::TypeCastError("string"))?.borrow().clone();
        let command = BuilderCommand::Uniform(name);
        let builder = find_frp_builder(scope)?;
        builder.push_command(command);
        Ok(Variable::Nil)
    })
}