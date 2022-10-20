mod main;
mod error;
mod builtin;
mod script_context;
mod render_path;
mod node;
mod atom;
mod posteffect_stack;
mod script_plugin;
pub mod nodes;
mod win_event;
mod node_list;
mod ubo_array_collect;
pub use script_context::{ScriptContext};
pub use main::{RenderMain};
use smol_str::SmolStr;
pub use ubo_array_collect::{UBOArrayCollect};
pub use node::{NodeCreatorSet,NodeCreatorFn,IUpdateNode,UpdateNodeBox};
pub use  posteffect_stack::{PostEffectStack};

pub struct RenderScriptPlugin {
    pub node_creators:NodeCreatorSet,
    script_mod:Option<(SmolStr,String)>
}

impl RenderScriptPlugin {
    pub fn new(node_creators:NodeCreatorSet) -> Self {
        RenderScriptPlugin {
            node_creators,
            script_mod:None
        }
    }

    pub fn set_script_mod(&mut self,mod_name:&str,code_string:String) {
        self.script_mod = Some((mod_name.into(),code_string));
    }

    
}