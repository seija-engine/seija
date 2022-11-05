use lite_clojure_eval::{EvalRT, ExecScope, Variable, run_native_fn};
use serde_json::Value;
use anyhow::{anyhow,Result};
use std::{convert::TryFrom, sync::Arc};
use crate::{UniformInfo, UniformInfoSet};
use super::errors::Errors;
use super::builder::{FRPCompBuilder, BuilderCommand};


pub fn init_fns(vm:&mut EvalRT) {
    
    vm.global_context().push_native_fn("declare-uniform", declare_uniform);
    vm.global_context().push_native_fn("__frp_enter__", __frp_enter__);
    vm.global_context().push_native_fn("__frp_exit__", __frp_exit__);
    vm.global_context().push_native_fn("uniform", uniform);
    vm.global_context().push_native_fn("node", node);

    vm.global_context().push_var("SS_VERTEX", wgpu::ShaderStage::VERTEX.bits() as i64 );
    vm.global_context().push_var("SS_FRAGMENT", wgpu::ShaderStage::FRAGMENT.bits() as i64 );
    vm.global_context().push_var("SS_VERTEX_FRAGMENT", wgpu::ShaderStage::VERTEX_FRAGMENT.bits() as i64 );
    vm.global_context().push_var("SS_COMPUTE", wgpu::ShaderStage::COMPUTE.bits() as i64 );
    vm.global_context().push_var("SS_ALL", wgpu::ShaderStage::all().bits() as i64 );
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

pub fn __frp_exit__(s:&mut ExecScope,a:Vec<Variable>) -> Variable {
    run_native_fn("__frp_exit__", s, a, |scope,_| {
        let command = BuilderCommand::EndComp;
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


pub fn node(s:&mut ExecScope,a:Vec<Variable>) -> Variable { 
    run_native_fn("node", s, a, |scope,mut args| {
        let node_id = args.remove(0).cast_int().ok_or(Errors::TypeCastError("int") )?;
        let builder = find_frp_builder(scope)?;
        let command = BuilderCommand::Node(node_id,args);
        builder.push_command(command);
        Ok(Variable::Nil)
    })
}