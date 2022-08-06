use seija_render::{RenderScriptPlugin, NodeCreatorSet};


pub fn create_skeleton_plugin() -> RenderScriptPlugin {
    let node_creators = NodeCreatorSet::default();
    //node_creators.0.insert("SKELETON_SKIN".into(), create_skeleton_skin);
    RenderScriptPlugin::new(node_creators)
}

