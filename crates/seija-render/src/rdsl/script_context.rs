use lite_clojure_eval::{EvalRT, Variable};
use seija_asset::Assets;
use crate::{UniformInfoSet, RenderContext, resource::Texture};

use super::builtin::{declare_uniform, add_uniform, select_add_uniform};

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
        self.rt.global_context().push_native_fn("declare-uniform", declare_uniform);
        self.rt.global_context().push_native_fn("add-uniform", add_uniform);
        self.rt.global_context().push_native_fn("select-add-uniform", select_add_uniform);
        self.rt.global_context().push_var("SS_VERTEX", wgpu::ShaderStage::VERTEX.bits() as i64 );
        self.rt.global_context().push_var("SS_FRAGMENT", wgpu::ShaderStage::FRAGMENT.bits() as i64 );
        self.rt.global_context().push_var("SS_VERTEX_FRAGMENT", wgpu::ShaderStage::VERTEX_FRAGMENT.bits() as i64 );
        self.rt.global_context().push_var("SS_COMPUTE", wgpu::ShaderStage::COMPUTE.bits() as i64 );
        self.rt.global_context().push_var("SS_ALL", wgpu::ShaderStage::all().bits() as i64 );
    }

    pub fn start(&mut self) {

    }

    pub fn exec_declare_uniform(&mut self,info_set:&mut UniformInfoSet) {
        let info_ptr = info_set as *mut UniformInfoSet as *mut u8;
        if let Err(err) = self.rt.invoke_func("declare-uniforms", vec![Variable::UserData(info_ptr)]) {
            log::error!("{:?}",err);
        }
    }

    pub fn exec_render_start(&mut self,ctx:&mut RenderContext,textures:&mut Assets<Texture>)  {
        self.set_userdata("*TEXTURES*", textures);
        self.set_userdata("*RENDER_CTX*", ctx);

        if let Err(err) = self.rt.invoke_func("on-render-start", vec![Variable::Nil]) {
            log::error!("{:?}",err);
        }
    }

    fn set_userdata<T>(&mut self,name:&str,value:&mut T) {
        let value_ptr = value as *mut T as *mut u8;
        if let Some(sym) = self.rt.main_context().find_local_symbol(name) {
            self.rt.main_context().set_var(name,Variable::UserData(value_ptr) );
        } else {
            self.rt.main_context().push_var(name, Variable::UserData(value_ptr));
        }
    }

}