use std::sync::Arc;
use std::convert::{TryFrom};
use lite_clojure_eval::{ExecScope, Variable};
use seija_asset::Assets;
use serde_json::Value;
use crate::resource::Texture;
use crate::{UniformInfoSet, UniformInfo, RenderContext};

fn find_userdata<T>(scope:&mut ExecScope,name:&str) -> Option<&'static mut T> {
    let textures = scope.context.find_local_symbol(name)?;
    let ptr = textures.cast_userdata()?;
    let value_ptr = unsafe { &mut *(ptr as *mut T) };
    Some(value_ptr)
}

pub fn declare_uniform(scope:&mut ExecScope,args:Vec<Variable>) -> Variable { 
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
    info.name =  Arc::new(name);
    info_set_mut.add_info(info);
    Some(())
}

pub fn add_uniform(scope:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if _add_uniform(scope, args).is_none() {
        log::error!("add uniform error");
    }
    Variable::Nil
}

fn _add_uniform(scope:&mut ExecScope,args:Vec<Variable>) -> Option<()> { 
    if args.len() != 1 { return None; }
    let ctx = find_userdata::<RenderContext>(scope,"*RENDER_CTX*")?;
    let name = args[0].cast_string()?;
    ctx.ubo_ctx.add_uniform(name.borrow().as_str(), &mut ctx.resources);
    Some(())
}


pub fn select_add_uniform(scope:&mut ExecScope,args:Vec<Variable>) -> Variable { 
    if _select_add_uniform(scope, args).is_none() {
        log::error!("select uniform error");
    }
    Variable::Nil
}

fn _select_add_uniform(_scope:&mut ExecScope,args:Vec<Variable>) -> Option<()> { 
    if args.len() != 2 { return None; }
    
    Some(())
}