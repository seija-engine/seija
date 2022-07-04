use lite_clojure_eval::{Variable, GcRefCell};
use seija_render::{RenderScriptPlugin, NodeCreatorSet, RenderGraphContext, graph::NodeId, UpdateNodeBox};

use crate::nodes::{PBRLightCollect, PBRCameraNode};

pub fn create_pbr_plugin() -> RenderScriptPlugin {
    let mut node_creators = NodeCreatorSet::default();
    //node_creators.0.insert("PBR_CAMERA_EX".into(), create_pbr_camera_ex);
    //node_creators.0.insert("PBRLIGHT".into(), create_pbr_light_collect);
    node_creators.add::<PBRCameraNode>("PBR_CAMERA_EX".into());
    RenderScriptPlugin::new(node_creators)
}



fn create_pbr_light_collect(ctx:&mut RenderGraphContext,params:Variable) -> NodeId {
    let map = params.cast_map().unwrap();
    let ubo_key = Variable::Keyword(GcRefCell::new(String::from(":ubo")));
    let ubo_name = map.borrow().get(&ubo_key).unwrap().cast_string().unwrap();
    let ubo_name_str:String = ubo_name.borrow().clone();

    let light_collect = PBRLightCollect::new(ubo_name_str);
    ctx.graph.add_node("PBRLightCollect", light_collect)
}