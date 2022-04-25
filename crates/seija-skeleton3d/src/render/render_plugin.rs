use lite_clojure_eval::{Variable, GcRefCell};
use seija_render::{RenderScriptPlugin, NodeCreatorSet, RenderGraphContext, graph::NodeId};

use super::skeleton_node::SkeletonNode;

pub fn create_skeleton_plugin() -> RenderScriptPlugin {
    let mut node_creators = NodeCreatorSet::default();
    node_creators.0.insert("SKELETON_SKIN".into(), create_skeleton_skin);
    RenderScriptPlugin::new(node_creators)
}

fn create_skeleton_skin(ctx:&mut RenderGraphContext,params:Variable) -> NodeId {
    let map = params.cast_map().unwrap();
    let ubo_key = Variable::Keyword(GcRefCell::new(String::from(":ubo")));
    let ubo_name = map.borrow().get(&ubo_key).unwrap().cast_string().unwrap();
    let ubo_name_str:String = ubo_name.borrow().clone();

    let skeleton_node = SkeletonNode::new(ubo_name_str);
    ctx.graph.add_node("SkeletonSkin", skeleton_node)
}