mod main;
mod error;
mod builtin;
mod rt_tags;
mod script_context;
mod render_path;
mod node;
pub mod nodes;
pub use script_context::{ScriptContext};
pub use main::{RenderMain};

pub use node::{NodeCreatorSet,NodeCreatorFn,IUpdateNode,UpdateNodeBox};

pub struct RenderScriptPlugin {
    pub node_creators:NodeCreatorSet
}

impl RenderScriptPlugin {
    pub fn new(node_creators:NodeCreatorSet) -> Self {
        RenderScriptPlugin {
            node_creators,
        }
    }
}