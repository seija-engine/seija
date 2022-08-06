use seija_render::{RenderScriptPlugin, NodeCreatorSet};



pub fn create_deferred_plugin() -> RenderScriptPlugin {
    let node_creators = NodeCreatorSet::default();
    //node_creators.0.insert("DEFERRED_LIGHT_PASS".into(), create_deferred_light_pass);
    RenderScriptPlugin::new(node_creators)
}