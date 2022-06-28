use seija_asset::Assets;

use crate::{UniformInfoSet, resource::Texture, RenderContext};

use super::ScriptContext;
//这里通过逻辑保证RenderMain只在一个线程运行，ECS库的System必须要这俩个trait
unsafe impl Send for RenderMain {}
unsafe impl Sync for RenderMain {}
pub struct RenderMain {
    script_ctx:ScriptContext,
}

impl RenderMain {
    pub fn new() -> Self {
        RenderMain { 
            script_ctx:ScriptContext::new()
        }
    }

    pub fn init(&mut self,code_string:&str,info_set:&mut UniformInfoSet) {
        self.script_ctx.init(code_string);
        self.script_ctx.exec_declare_uniform(info_set);
    }

    pub fn start(&mut self,textures:&mut Assets<Texture>,ctx:&mut RenderContext) {
       
        self.script_ctx.exec_render_start(ctx, textures);
    }
}



