use lite_clojure_eval::{Variable, GcRefCell};
use seija_render::{RenderScriptPlugin, NodeCreatorSet, RenderGraphContext, graph::NodeId};

use crate::gbuffer_node::GBufferNode;

pub fn create_deferred_plugin() -> RenderScriptPlugin {
    let mut node_creators = NodeCreatorSet::default();
    node_creators.0.insert("GBUFFER".into(), create_gbuffer);
    RenderScriptPlugin::new(node_creators)
}

fn create_gbuffer(ctx:&mut RenderGraphContext,params:Variable) -> NodeId {
    //let map = params.cast_map().unwrap();
    //let ubo_key = Variable::Keyword(GcRefCell::new(String::from(":ubo")));
    //let ubo_name = map.borrow().get(&ubo_key).unwrap().cast_string().unwrap();
    //let ubo_name_str:String = ubo_name.borrow().clone();

    let gbuffer = GBufferNode::new();
    ctx.graph.add_node("GBUFFER", gbuffer)
}