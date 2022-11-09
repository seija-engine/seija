use std::collections::HashMap;
use lite_clojure_eval::Variable;
use smol_str::SmolStr;
use anyhow::Result;
use super::elems::{camera_node::CameraNode, IUpdateNode, transform_node::TransfromNode, window_resize_node::WindowReSizeNode};

pub type NodeCreateFn = fn(args:Vec<Variable>) -> Result<Box<dyn IUpdateNode>>;
#[derive(Default)]
pub struct RenderScriptPlugin {
    pub node_creators:HashMap<SmolStr,NodeCreateFn>,
    pub events:Vec<SmolStr>,
    pub dynamics:Vec<(SmolStr,Variable)>,
}

impl RenderScriptPlugin {
    pub fn add_node_creator(&mut self,name:&str,creatorf:NodeCreateFn) {
        self.node_creators.insert(name.into(), creatorf);
    }
}

pub fn create_buildin_plugin() -> RenderScriptPlugin {
    let mut plugin = RenderScriptPlugin::default();
    plugin.add_node_creator("Camera", CameraNode::from_args);
    plugin.add_node_creator("Transform", TransfromNode::from_args);
    plugin.add_node_creator("WinResize", WindowReSizeNode::from_args);
    plugin.dynamics.push(("dynBase3D".into(),Variable::Bool(true)));
    plugin
}