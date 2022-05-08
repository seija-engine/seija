use lite_clojure_eval::{Variable,GcRefCell};
use seija_render::{RenderScriptPlugin, NodeCreatorSet, RenderGraphContext, graph::NodeId};
use serde_json::Value;

use crate::deferred_light_pass::DeferredLightPass;

pub fn create_deferred_plugin() -> RenderScriptPlugin {
    let mut node_creators = NodeCreatorSet::default();
    node_creators.0.insert("DEFERRED_LIGHT_PASS".into(), create_deferred_light_pass);
    RenderScriptPlugin::new(node_creators)
}

fn create_deferred_light_pass(ctx:&mut RenderGraphContext,params:Variable) -> NodeId {
    let json_value:Value = params.into();
    let map_value = json_value.as_object().unwrap();
    let tex_count = map_value.get(":tex-count").and_then(Value::as_i64).unwrap() as usize;
    let pass_node = DeferredLightPass::new(tex_count);
    ctx.graph.add_node("DeferredLightPass", pass_node)
}