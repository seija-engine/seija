use std::collections::HashMap;

use lite_clojure_eval::{Variable,GcRefCell};

use crate::{graph::{NodeId, nodes::CameraCollect}, render::RenderGraphContext};

use super::{NodeCreatorSet, NodeCreatorFn};

pub fn builtin_node_creators() -> NodeCreatorSet {
    let mut map:HashMap<String,NodeCreatorFn> = HashMap::default();
    map.insert("CAMERA".into(), create_camera_node);
    NodeCreatorSet(map)
}

fn create_camera_node(ctx:&mut RenderGraphContext,params:Variable) -> NodeId {
    let map = params.cast_map().unwrap();
    let ubo_key = Variable::Keyword(GcRefCell::new(String::from(":ubo")));
    let ubo_name = map.borrow().get(&ubo_key).unwrap().cast_string().unwrap();
    let ubo_name_str:String = ubo_name.borrow().clone();
    let camera_collect = CameraCollect {ubo_name:ubo_name_str};
    dbg!(&camera_collect.ubo_name);
    ctx.graph.add_node("CameraCollect", camera_collect)
}