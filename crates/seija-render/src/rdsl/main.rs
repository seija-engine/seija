use crate::{UniformInfoSet};

use super::ScriptContext;

pub struct RenderMain {
    uniforms:UniformInfoSet
}

impl RenderMain {
    pub fn new() -> Self {
        RenderMain { 
            uniforms: UniformInfoSet::default() 
        }
    }

    pub fn init(&mut self) {
        let mut script_ctx:ScriptContext = ScriptContext::new();
        script_ctx.exec_declare_uniform(&mut self.uniforms);
    }
}



