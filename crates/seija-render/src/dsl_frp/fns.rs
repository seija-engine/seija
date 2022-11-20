use bevy_ecs::prelude::Entity;
use bevy_ecs::world::World;
use lite_clojure_eval::parser::cexpr::CExpr;
use lite_clojure_eval::parser::value::Symbol;
use lite_clojure_eval::{EvalRT, ExecScope, Variable, run_native_fn,parser};
use lite_clojure_frp::{FRPSystem, DynamicID};
use seija_core::OptionExt;
use serde_json::Value;
use anyhow::{anyhow,Result};
use std::{convert::TryFrom, sync::Arc};
use crate::material::{STextureDescriptor};
use crate::query::IdOrName;
use crate::resource::TextureDescInfo;
use crate::{UniformInfo, UniformInfoSet};
use super::errors::Errors;
use super::builder::{FRPCompBuilder, BuilderCommand};
use super::render_path::{RenderPathDefine, RenderPathContext};



pub fn add_macros() {
    parser::add_macro_func(frp_macros);
    
}


fn frp_macros(expr:&mut CExpr) {
    match expr {
        CExpr::List(lst) => {
            match lst.first() {
                
                Some(CExpr::Symbol(sym)) => {
                    match sym.name.as_str() {
                        "fc" => { ex_fc(lst) },
                        "defcomp" => { ex_defcomp(lst) },
                        _ => { lst.iter_mut().for_each(frp_macros); }
                    }
                }
                _ => { lst.iter_mut().for_each(frp_macros); }
            }
        },
        CExpr::Vector(lst) => { lst.iter_mut().for_each(frp_macros); },
        CExpr::Map(lst) => { lst.iter_mut().for_each(frp_macros); },
        _ => {}
    }
}

fn ex_fc(lst:&mut Vec<CExpr>) {
    //(fc [args] expr1 expr2) to (fn [args]  (__frp_enter__ "start") expr1 expr2 (__frp_exit__))
    lst.remove(0);
    let fn_expr = CExpr::Symbol(Symbol::intern(None, "fn".into()));
    let enter_expr = CExpr::Symbol(Symbol::intern(None, "__frp_enter__".into()));
    let exit_expr = CExpr::Symbol(Symbol::intern(None, "__frp_exit__".into()));
    let enter_fn = CExpr::List(vec![enter_expr]);
    let exit_fn = CExpr::List(vec![exit_expr]);
    lst.insert(0,fn_expr);
    lst.insert(2, enter_fn);
    lst.push(exit_fn);
    lst.iter_mut().for_each(frp_macros);
}

fn ex_defcomp(lst:&mut Vec<CExpr>) {
    /*
    (defcomp fnname [args] expr1 expr2) to 
    (def fnname (fn [args]
        (__frp_enter__ "start")
        expr1
        expr2
        (__frp_exit__)
    ))
    */
    lst.remove(0);
    let fn_name_expr = lst.remove(0);
    let fn_name_string = fn_name_expr.clone().cast_string().unwrap_or_default();

    let fn_expr = CExpr::Symbol(Symbol::intern(None, "fn".into()));
    lst.insert(0, fn_expr);
    let enter_expr = CExpr::Symbol(Symbol::intern(None, "__frp_enter__".into()));
    let exit_expr = CExpr::Symbol(Symbol::intern(None, "__frp_exit__".into()));
    let enter_fn = CExpr::List(vec![enter_expr,CExpr::String(fn_name_string)]);
    let exit_fn = CExpr::List(vec![exit_expr]);
    lst.insert(2, enter_fn);
    lst.push(exit_fn);
    lst.iter_mut().for_each(frp_macros);

    let def_expr = CExpr::Symbol(Symbol::intern(None, "def".into()));
    let new_expr = vec![def_expr,fn_name_expr, CExpr::List(lst.drain(..).collect())];
    *lst = new_expr;
}

pub fn init_fns(vm:&mut EvalRT) {
    
    vm.global_context().push_native_fn("declare-uniform", declare_uniform);
    vm.global_context().push_native_fn("__frp_enter__", __frp_enter__);
    vm.global_context().push_native_fn("__frp_exit__", __frp_exit__);
    vm.global_context().push_native_fn("uniform", uniform);
    vm.global_context().push_native_fn("node", node);
    vm.global_context().push_native_fn("texture", texture);
    vm.global_context().push_native_fn("if-comp", if_comp);
    vm.global_context().push_native_fn("add-render-path", add_render_path);
    vm.global_context().push_native_fn("posteffect-item", posteffect_item);
    vm.global_context().push_native_fn("add-query", add_query);
    vm.global_context().push_native_fn("uniform-set", uniform_set);

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

fn find_world(scope:&mut ExecScope) -> Result<&'static mut World> {
    find_userdata::<World>(scope, "*WORLD*")
}

fn find_frp_system(scope:&mut ExecScope) -> Result<&'static mut FRPSystem> {
    find_userdata::<FRPSystem>(scope, "*FRPSystem*")
}

pub fn __frp_enter__(s:&mut ExecScope,a:Vec<Variable>) -> Variable {
    run_native_fn("__frp_enter__", s, a, |scope,args| {
        let name:String = if args.len() > 0 {
            args[0].cast_string().ok_or(Errors::TypeCastError("string"))?.borrow().clone()
        } else { "".into() }; 
        
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
    run_native_fn("texture", s, a, |scope,args| {
        let value:Value = args.get(0).ok_or(Errors::FuncParamCountError)?.clone().into();
        let steture_desc = STextureDescriptor::try_from(&value).map_err(|err| anyhow!("{:?}",err))?;
        let mut desc_info = TextureDescInfo::default();
        desc_info.desc = steture_desc.0;

        let dyn_texture = {
           let frp_sys = find_frp_system(scope)?;
           frp_sys.new_dynamic(Variable::Nil, frp_sys.never(), None).unwrap()
        };

        let builder = find_frp_builder(scope)?;
        builder.push_command(BuilderCommand::Texture(desc_info,dyn_texture));

        Ok(Variable::Int(dyn_texture as i64))
    })
}

pub fn if_comp(s:&mut ExecScope,a:Vec<Variable>) -> Variable { 
    run_native_fn("if-comp", s, a, |scope,mut args| {
        if args.len() < 2 || args.len() > 3 { return Err(anyhow!(Errors::FuncParamCountError)); }
        let dyn_id = args.remove(0).cast_int().ok_or(Errors::TypeCastError("int"))? as DynamicID;
        let true_comp = args.remove(0);
        let else_comp = if args.len() > 0 { Some(args.remove(0)) } else { None };
        let builder = find_frp_builder(scope)?;
        builder.push_command(BuilderCommand::IfComp(dyn_id,true_comp,else_comp));
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

pub fn posteffect_item(s:&mut ExecScope,a:Vec<Variable>) -> Variable {
    run_native_fn("posteffect-item", s, a, |scope,args| {
        let entity_id = args.get(0).and_then(Variable::cast_int).get()?;
        let entity = Entity::from_bits(entity_id as u64);
        let material_path = args.get(1).and_then(Variable::cast_string).get()?;
        let sort_order = args.get(2).and_then(Variable::cast_int).get()? as u32;
        let builder = find_frp_builder(scope)?;
        builder.push_command(BuilderCommand::PostEffectItem(entity,material_path.borrow().as_str().into(),sort_order));
        
        Ok(Variable::Nil)
    })
}

pub fn add_query(s:&mut ExecScope,a:Vec<Variable>) -> Variable {
    run_native_fn("add-query", s, a, |scope,args| {
        let id_or_name = get_query_name(args.get(0).get()?)?;
        let query_type = args.get(1).and_then(Variable::cast_int).get()? as u32;

        let dyn_query = {
            let frp_sys = find_frp_system(scope)?;
            frp_sys.new_dynamic(Variable::Nil, frp_sys.never(), None).get()?
        };
        let builder = find_frp_builder(scope)?;
        builder.push_command(BuilderCommand::AddQuery(dyn_query, id_or_name, query_type));
        Ok(Variable::Int(dyn_query as i64))
    })
}

fn get_query_name(var:&Variable) -> Result<IdOrName> {
    match var {
        Variable::String(s) => { Ok(IdOrName::Name(s.borrow().clone())) },
        Variable::Int(i) => { Ok(IdOrName::Id(*i as u64)) },
        _ => { return Err(anyhow!("query type error")); }
    }
}

pub fn uniform_set(s:&mut ExecScope,a:Vec<Variable>) -> Variable {
    run_native_fn("uniform-set", s, a, |scope,args| {
        let entity_id = args.get(0).and_then(Variable::cast_int).map(|v| Entity::from_bits(v as u64));
        let uniform_name = args.get(1).and_then(Variable::cast_string).get()?;
        let prop_name = args.get(2).and_then(Variable::cast_string).get()?;
        let dyn_id = args.get(3).and_then(Variable::cast_int).get()? as DynamicID;

        let builder = find_frp_builder(scope)?;
        builder.push_command(BuilderCommand::UniformSet(entity_id, uniform_name.borrow().as_str().into(), 
                                                        prop_name.borrow().as_str().into(),dyn_id));
        Ok(Variable::Nil)
    })
}
