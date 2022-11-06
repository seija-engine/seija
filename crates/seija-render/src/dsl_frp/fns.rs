use lite_clojure_eval::{EvalRT, ExecScope, Variable, run_native_fn};
use serde_json::Value;
use anyhow::{anyhow,Result};
use std::{convert::TryFrom, sync::Arc};
use crate::material::STextureDescriptor;
use crate::resource::TextureDescInfo;
use crate::{UniformInfo, UniformInfoSet};
use super::errors::Errors;
use super::builder::{FRPCompBuilder, BuilderCommand};
use super::render_path::{RenderPathDefine, RenderPathContext};


pub fn init_fns(vm:&mut EvalRT) {
    
    vm.global_context().push_native_fn("declare-uniform", declare_uniform);
    vm.global_context().push_native_fn("__frp_enter__", __frp_enter__);
    vm.global_context().push_native_fn("__frp_exit__", __frp_exit__);
    vm.global_context().push_native_fn("uniform", uniform);
    vm.global_context().push_native_fn("node", node);
    vm.global_context().push_native_fn("texture", texture);
    vm.global_context().push_native_fn("add-render-path", add_render_path);

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

pub fn texture(s:&mut ExecScope,a:Vec<Variable>) -> Variable { 
    run_native_fn("texture", s, a, |scope,mut args| {
        let builder = find_frp_builder(scope)?;
        let value:Value = args.get(0).ok_or(Errors::FuncParamCountError)?.clone().into();
        let steture_desc = STextureDescriptor::try_from(&value).map_err(|err| anyhow!("{:?}",err))?;
        let mut desc_info = TextureDescInfo::default();
        desc_info.desc = steture_desc.0;
        Ok(Variable::Nil)
    })
}

pub fn add_render_path(s:&mut ExecScope,a:Vec<Variable>) -> Variable { 
    run_native_fn("add-render-path", s, a, |scope,mut args| {
        let path_name = args.remove(0).cast_string().ok_or(Errors::TypeCastError("string"))?;
        if let Ok(path_ctx) = find_userdata::<RenderPathContext>(scope, "*PATH_DEFINE_SET*") {
            let frp_fn = args.remove(0);
            let path_define = RenderPathDefine {name:path_name.borrow().as_str().into(),start_func:frp_fn};
            path_ctx.add_define(path_define);
        }
        Ok(Variable::Nil)
    })
}