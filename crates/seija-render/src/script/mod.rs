mod script_context;
mod node_creator;
mod builtin;

pub use builtin::{builtin_node_creators};
pub use script_context::{RenderScriptContext};
pub use node_creator::*;
pub struct RenderScriptPlugin {
    pub node_creators:NodeCreatorSet,
    pub script_mod_name:Option<String>,
    pub script_mod_code:Option<String>
}

impl RenderScriptPlugin {
    pub fn new(node_creators:NodeCreatorSet,script_mod_name:Option<String>,script_mod_code:Option<String>) -> Self {
        RenderScriptPlugin {
            node_creators,
            script_mod_name,
            script_mod_code
        }
    }
}


