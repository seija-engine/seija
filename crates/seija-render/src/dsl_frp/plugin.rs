use std::{collections::HashMap, sync::Arc};
use lite_clojure_eval::Variable;
use smol_str::SmolStr;
use anyhow::Result;
use crate::shadow::ShadowNode;

use super::elems::{camera_node::CameraNode, IUpdateNode, transform_node::TransfromNode, 
    window_resize_node::WindowReSizeNode, draw_pass_node::DrawPassNode, post_stack_node::PostStackNode, ibl_node::IBLNode};

pub type NodeCreateFn = fn(args:Vec<Variable>) -> Result<Box<dyn IUpdateNode>>;

pub enum ApplyCameraType {
    ALL,
    Path(SmolStr)
}

#[derive(Default)]
pub struct RenderScriptPlugin {
    pub node_creators:HashMap<SmolStr,NodeCreateFn>,
    pub global_events:Vec<SmolStr>,
    pub global_dynamics:Vec<(SmolStr,Variable)>,

    pub camera_events:Arc<Vec<(ApplyCameraType,SmolStr)>>,
    pub camera_dynamics:Arc<Vec<(ApplyCameraType,SmolStr,Variable)>>
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
    plugin.add_node_creator("DrawPass", DrawPassNode::from_args);
    plugin.add_node_creator("PostStack", PostStackNode::from_args);
    plugin.add_node_creator("Shadow", ShadowNode::from_args);
    plugin.add_node_creator("IBL", IBLNode::from_args);

    plugin.global_dynamics.push(("dynBase3D".into(),Variable::Bool(true)));
    plugin.global_dynamics.push(("dynShadow".into(),Variable::Bool(false)));

    let mut camera_dyns = vec![];
    camera_dyns.push((ApplyCameraType::ALL,":dynIsHDR".into(),Variable::Bool(false)));
    camera_dyns.push((ApplyCameraType::ALL,":dynHasPostEffect".into(),Variable::Bool(false)));
    plugin.camera_dynamics = Arc::new(camera_dyns);
    plugin
}