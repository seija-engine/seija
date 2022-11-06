use seija_render::dsl_frp::RenderScriptPlugin;
use crate::{elems::{pbr_camera::PBRCameraNode, pbr_light::PBRLightNode}};

pub fn create_pbr_plugin() -> RenderScriptPlugin {
   let mut plugin = RenderScriptPlugin::default();
   plugin.add_node_creator("PBRCameraEx", PBRCameraNode::from_args);
   plugin.add_node_creator("PBRLight", PBRLightNode::from_args);
   plugin
}