use std::sync::Arc;
use std::convert::{TryFrom};
use lite_clojure_eval::{ExecScope, Variable, GcRefCell};
use serde_json::Value;
use crate::rdsl::main::DynUniformItem;
use crate::{UniformInfoSet, UniformInfo, RenderContext};

use super::main::MainContext;
use super::render_path::RenderPathDef;

pub fn find_userdata<T>(scope:&mut ExecScope,name:&str) -> Option<&'static mut T> {
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
    if !ctx.ubo_ctx.add_uniform(name.borrow().as_str(), &mut ctx.resources) {
        log::error!("not found uniform:{}",name.borrow().as_str());
    }
    Some(())
}


pub fn select_add_uniform(scope:&mut ExecScope,args:Vec<Variable>) -> Variable { 
    if _select_add_uniform(scope, args).is_none() {
        log::error!("select uniform error");
    }
    Variable::Nil
}

fn _select_add_uniform(scope:&mut ExecScope,args:Vec<Variable>) -> Option<()> { 
    if args.len() != 2 { return None; }
    let main_ctx = find_userdata::<MainContext>(scope, "*MAIN_CTX*")?;
    let ctx = find_userdata::<RenderContext>(scope,"*RENDER_CTX*")?;

    let tag_name = args[0].cast_string()?;
    let tag_index = main_ctx.rt_tags.name_id(tag_name.borrow().as_str())?;
    let ubo_name:String = args[1].cast_string()?.borrow().clone();
    let enable =  main_ctx.rt_tags.tags[tag_index];
    if enable {
        if !ctx.ubo_ctx.add_uniform(&ubo_name, &mut ctx.resources) {
            log::error!("not found uniform:{}",ubo_name.as_str());
        }
    }
    let item = DynUniformItem {tag_index,ubo_name,enable};
    main_ctx.dyn_uniform_set.push(item);

    Some(())
}

pub fn add_tag(scope:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if args.len() != 2 {
        log::error!("add-tag args error !=2"); 
        return Variable::Nil; 
    }
    if let Some(main_ctx) = find_userdata::<MainContext>(scope, "*MAIN_CTX*") {
        match (args[0].cast_string(),args[1].cast_bool()) {
            (Some(name),Some(b)) => {
                main_ctx.rt_tags.add_tag(name.borrow().as_str(),b);
            }
            _ => { log::error!("add-tag error"); }
        }
    } else {
        log::error!("*MAIN_CTX* is nil");
    }
    Variable::Nil
}

pub fn add_render_path(scope:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if _add_render_path(scope, args).is_none() {
        log::error!("add render path error");
    }
    Variable::Nil
}

pub fn _add_render_path(scope:&mut ExecScope,args:Vec<Variable>) -> Option<()> {
    if args.len() != 2 {
        log::error!("add-render-paths args error !=2"); 
        return None; 
    }
    let main_ctx = find_userdata::<MainContext>(scope, "*MAIN_CTX*")?;
    let name = args[0].cast_string()?.borrow().clone();
    let map = args[1].cast_map()?;
    let start_key = Variable::Keyword(GcRefCell::new(String::from(":on-start")  ));
    let update_key = Variable::Keyword(GcRefCell::new(String::from(":on-update")  ));
    let start_fn = map.borrow().get(&start_key)?.clone();
    let update_fn = map.borrow().get(&update_key)?.clone();
    let render_path = RenderPathDef {
        name,
        start_fn,
        update_fn
    };
    main_ctx.path_dic.insert(render_path.name.clone(), render_path);
    Some(())
}