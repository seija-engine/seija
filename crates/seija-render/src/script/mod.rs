mod script_context;
mod node_creator;
mod builtin;

pub use builtin::{builtin_node_creators};
pub use script_context::{RenderScriptContext};
pub use node_creator::*;
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


