//use seija_render::{RenderScriptPlugin, NodeCreatorSet};

use seija_render::dsl_frp::RenderScriptPlugin;


pub fn create_skeleton_plugin() -> RenderScriptPlugin {
    let plugin = RenderScriptPlugin::default();

    plugin
    //let node_creators = NodeCreatorSet::default();
    //node_creators.0.insert("SKELETON_SKIN".into(), create_skeleton_skin);
    //RenderScriptPlugin::new(node_creators)
}

