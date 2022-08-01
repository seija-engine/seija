use bevy_ecs::prelude::World;
use lite_clojure_eval::{EvalRT, Variable};
use seija_asset::Assets;
use seija_core::window::AppWindow;
use crate::{UniformInfoSet, RenderContext, resource::Texture, query::QuerySystem};

use super::{builtin::*, main::{MainContext}};

pub struct ScriptContext {
   pub rt:EvalRT
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
        //let rt = &mut self.rt;
        //let rt_ptr = rt as *mut EvalRT as *mut u8;
        //self.rt.global_context().set_var("*VM*", Variable::UserData(rt_ptr) );
        self.rt.global_context().push_native_fn("declare-uniform", declare_uniform);
        self.rt.global_context().push_native_fn("add-uniform", add_uniform);
        self.rt.global_context().push_native_fn("select-add-uniform", select_add_uniform);
        self.rt.global_context().push_native_fn("add-tag", add_tag);
        self.rt.global_context().push_native_fn("add-render-path", add_render_path);
        self.rt.global_context().push_native_fn("add-node", add_node);
        self.rt.global_context().push_native_fn("atom-texture", atom_texture);
        self.rt.global_context().push_native_fn("add-query", add_query);
        self.rt.global_context().push_native_fn("get-query", get_query);
        self.rt.global_context().push_native_fn("set-global-uniform", set_global_uniform);
        self.rt.global_context().push_native_fn("tag?", is_tag);
        
        self.rt.global_context().push_var("SS_VERTEX", wgpu::ShaderStage::VERTEX.bits() as i64 );
        self.rt.global_context().push_var("SS_FRAGMENT", wgpu::ShaderStage::FRAGMENT.bits() as i64 );
        self.rt.global_context().push_var("SS_VERTEX_FRAGMENT", wgpu::ShaderStage::VERTEX_FRAGMENT.bits() as i64 );
        self.rt.global_context().push_var("SS_COMPUTE", wgpu::ShaderStage::COMPUTE.bits() as i64 );
        self.rt.global_context().push_var("SS_ALL", wgpu::ShaderStage::all().bits() as i64 );
    }

   
    pub fn exec_declare_uniform(&mut self,info_set:&mut UniformInfoSet) {
        let info_ptr = info_set as *mut UniformInfoSet as *mut u8;
        if let Err(err) = self.rt.invoke_func("declare-uniforms", vec![Variable::UserData(info_ptr)]) {
            log::error!("{:?}",err);
        }
    }

    pub fn set_global_const(&mut self,world:&World) {
        if let Some(window) = world.get_resource::<AppWindow>() {
            self.rt.global_context().set_var("WINDOW_WIDTH", Variable::Int(window.width() as i64));
            self.rt.global_context().set_var("WINDOW_HEIGHT", Variable::Int(window.height() as i64));
        }
    }

    pub fn set_script_global(&mut self,ctx:&mut RenderContext,main_ctx:&mut MainContext,world:&mut World) {
        unsafe {
            let mut textures = world.get_resource_unchecked_mut::<Assets<Texture>>().unwrap();
            let textures_mut = textures.as_mut();
    
            let mut query_system = world.get_resource_unchecked_mut::<QuerySystem>().unwrap();
            let query_system_mut = query_system.as_mut();
            
            self.set_userdata("*TEXTURES*", textures_mut);
            self.set_userdata("*QUERY*",  query_system_mut);
            self.set_userdata("*RENDER_CTX*", ctx);
            self.set_userdata("*MAIN_CTX*", main_ctx);
        }
    }

    pub fn exec_render_start(&mut self,
        ctx:&mut RenderContext,
        world:&mut World,
        main_ctx:&mut MainContext)  {
        self.set_script_global(ctx,main_ctx, world);
        if let Err(err) = self.rt.invoke_func("on-render-start", vec![Variable::Map(main_ctx.global_env.clone())]) {
            log::error!("{:?}",err);
        }
    }

    pub fn set_userdata<T>(&mut self,name:&str,value:&mut T) {
        let value_ptr = value as *mut T as *mut u8;
        self.rt.main_context().set_var(name,Variable::UserData(value_ptr) );
    }
}