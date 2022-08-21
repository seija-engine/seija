use std::sync::Arc;
use std::convert::{TryFrom};
use bevy_ecs::world::World;
use lite_clojure_eval::{ExecScope, Variable, GcRefCell};
use seija_asset::{Assets, AssetServer};
use serde_json::Value;
use crate::material::{STextureDescriptor, MaterialDefineAsset};
use crate::query::{QuerySystem, IdOrName};
use crate::rdsl::main::DynUniformItem;
use crate::rdsl::nodes::CameraNode;
use crate::resource::{Texture, TextureDescInfo, RenderResourceId};
use crate::shadow::ShadowNode;
use crate::{UniformInfoSet, UniformInfo, RenderContext};

use super::atom::Atom;
use super::main::MainContext;
use super::node::{NodeCreatorSet, UpdateNodeBox};
use super::nodes::{TransfromNode, WindowReSizeNode, DrawPassNode, DrawQuadNode};
use super::render_path::RenderPathDef;


pub fn create_builtin_node_set() -> NodeCreatorSet {
    let mut node_set = NodeCreatorSet::default();
    node_set.add::<CameraNode>("CAMERA_NODE");
    node_set.add::<TransfromNode>("TRANSFROM_NODE");
    node_set.add::<WindowReSizeNode>("WINSIZE_TEXTURE");
    node_set.add::<DrawPassNode>("DRAW_PASS");
    node_set.add::<ShadowNode>("SHADOW_NODE");
    node_set.add::<DrawQuadNode>("DRAW_QUAD");
    node_set
}

fn handle_error(name:&str,scope:&mut ExecScope,args:Vec<Variable>,f:fn(&mut ExecScope,args:Vec<Variable>) -> Result<Variable,i32>) -> Variable {
    match f(scope,args) {
        Ok(var) => var,
        Err(err) => {
            log::error!("eval {} error:{}",name,err);
            Variable::Nil
        }
    }
}

pub fn find_userdata<T>(scope:&mut ExecScope,name:&str) -> Option<&'static mut T> {
    let textures = scope.context.find_symbol(None, name, &scope.modules)?;
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
    let start_fn = map.borrow().get(&start_key)?.clone();
    let render_path = RenderPathDef {
        name,
        start_fn,
    };
    main_ctx.path_list.path_dic.insert(render_path.name.clone(), Arc::new(render_path));
    
    Some(())
}

pub fn add_node(scope:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if let Err(err_code) = _add_node(scope, args) { log::error!("add node error:{}",err_code); }
    Variable::Nil
}

pub fn _add_node(scope:&mut ExecScope,mut args:Vec<Variable>) -> Result<(),i32> {
    if args.len() < 3 { log::error!("add node args error < 3");  return Err(0); }
    let env_map = args.remove(0).cast_map().ok_or (1)?;
    let nodes_key = Variable::Keyword(GcRefCell::new(String::from(":nodes")));
    let nodes_ptr = env_map.borrow().get(&nodes_key).ok_or(2)?.cast_userdata().ok_or(3)?;
   
    let nodes_mut = unsafe { &mut *(nodes_ptr as *mut Vec<UpdateNodeBox>) };
   
    let tag_name = args.remove(0).cast_string().map(|v| v.borrow().clone());
    let node_index = args.remove(0).cast_int().ok_or(5)?;
    let main_ctx = find_userdata::<MainContext>(scope, "*MAIN_CTX*").ok_or(6)?;
    let mut update_node = main_ctx.create_node(node_index as usize, args).ok_or(7)?;
    if let Some(tag_name) = tag_name.as_ref() {
        if let Some(tag_index) = main_ctx.rt_tags.name_id(tag_name.as_str()) {
            update_node.tag_index = Some(tag_index);
            update_node.enable = main_ctx.rt_tags.tags[tag_index];
        } else {
            log::error!("not found tag:{}",tag_name.as_str());
        }
    }
    nodes_mut.push(update_node);
    Ok(())
}

pub fn atom_texture(scope:&mut ExecScope,args:Vec<Variable>) -> Variable {
    match _atom_texture(scope, args) {
        Ok(var) => var,
        Err(err) => {
            log::error!("atom texture error:{:?}",err);
            Variable::Nil
        }
    }
}

pub fn _atom_texture(scope:&mut ExecScope, args:Vec<Variable>) -> Result<Variable,i32> {
    let value:Value = args.get(0).ok_or(0)?.clone().into();
    let texture_desc = STextureDescriptor::try_from(&value).map_err(|_|1)?;
    let mut desc_info = TextureDescInfo::default();
    desc_info.desc = texture_desc.0;
    let world:&mut World = find_userdata(scope, "*WORLD*").ok_or(2)?;
    let mut textures = world.get_resource_mut::<Assets<Texture>>().unwrap();
    let texture = Texture::create_by_desc(desc_info);
    let h_texture = textures.add(texture);
    let atom_texture = Box::new(Atom::new(RenderResourceId::Texture(h_texture)));
    let atom_ptr = Box::into_raw(atom_texture) as *mut u8;
    Ok(Variable::UserData( atom_ptr))
}

pub fn add_query(scope:&mut ExecScope,args:Vec<Variable>) -> Variable {
    match _add_query(scope, args) {
        Ok(_) => Variable::Bool(true),
        Err(err) => {
            log::error!("add-query error:{:?}",err);
            Variable::Bool(false)
        }
    }
}

pub fn _add_query(scope:&mut ExecScope, args:Vec<Variable>) -> Result<(),i32> {
    let query:&mut QuerySystem = find_userdata(scope, "*QUERY*").ok_or(0)?;
    let query_name = get_query_name(&args[0])?;
    let query_type = args[1].cast_int().ok_or(1)? as u32;
    query.add_query(query_name, query_type);
    Ok( () )
}

fn get_query_name(var:&Variable) -> Result<IdOrName,i32> {
    match var {
        Variable::String(s) => { Ok(IdOrName::Name(s.borrow().clone())) },
        Variable::Int(i) => { Ok(IdOrName::Id(*i as u64)) },
        _ => { return Err(1); }
    }
}

pub fn get_query(scope:&mut ExecScope,args:Vec<Variable>) -> Variable {
    let ret = _get_query(scope, args);
    match ret {
        Ok(v) => v,
        Err(err) => {
            log::error!("get-query error:{:?}",err);
            Variable::Nil
        }
    }
}

pub fn _get_query(scope:&mut ExecScope, args:Vec<Variable>) -> Result<Variable,i32> {
    let query:&mut QuerySystem = find_userdata(scope, "*QUERY*").ok_or(0)?;
    let query_name = get_query_name(&args[0])?;
    let index = query.get(query_name);
    if let Some(index) = index {
        return Ok(Variable::Int(index as i64));
    }
    Ok(Variable::Nil)
}

pub fn set_global_uniform(scope:&mut ExecScope,args:Vec<Variable>) -> Variable {
    if let Err(err) = _set_global_uniform(scope, args) {
        log::error!("set-global-uniform error:{:?}",err);
    }
    Variable::Nil
}

pub fn _set_global_uniform(scope:&mut ExecScope, args:Vec<Variable>) -> Result<Variable,i32> {
    let ctx = find_userdata::<RenderContext>(scope,"*RENDER_CTX*").ok_or(0)?;
    let ubo_name = args[0].cast_string().ok_or(1)?;
    let texture_name = args[1].cast_string().ok_or(2)?;
    let atom_texture = args[2].cast_userdata().ok_or(3)? as *mut Atom<RenderResourceId>;
    let atom_texture_ref = unsafe { &*atom_texture };
    if let RenderResourceId::Texture(texture) = atom_texture_ref.inner() {
       if let Err(err) = ctx.ubo_ctx.set_texture(None, ubo_name.borrow().as_str(), texture_name.borrow().as_str(), texture.clone_weak()) {
            log::error!("set-global-uniform err:{}",err);
       }
    }
    Ok(Variable::Nil)
}

pub fn is_tag(scope:&mut ExecScope,args:Vec<Variable>) -> Variable {
    match _is_tag(scope, args) {
        Err(err) => { log::error!("tag? error:{:?}",err); Variable::Bool(false) }
        Ok(v) => v
    }

}

pub fn _is_tag(scope:&mut ExecScope,args:Vec<Variable>) -> Result<Variable,i32> {
    let main_ctx = find_userdata::<MainContext>(scope, "*MAIN_CTX*").ok_or(0)?;
    let tag_name = args.get(0).and_then(Variable::cast_string).ok_or(1)?;
    let tag_value = main_ctx.rt_tags.get_tag(tag_name.borrow().as_str()).ok_or(2)?;
    Ok(Variable::Bool(tag_value))
}

pub fn load_material(scope:&mut ExecScope,args:Vec<Variable>) -> Variable {
   handle_error("load-material", scope, args, |scope,args| {
        let world = find_userdata::<World>(scope, "*WORLD*").ok_or(0)?;
        let server = world.get_resource::<AssetServer>().ok_or(1)?.clone();
        let path = args.get(0).and_then(Variable::cast_string).ok_or(2)?;
        let ret = server.load_sync::<MaterialDefineAsset>(world, path.borrow().as_str(), None,false);
        Ok(Variable::Bool(ret.is_some()))
   })
}