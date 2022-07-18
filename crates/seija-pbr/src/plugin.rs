use seija_render::{RenderScriptPlugin, NodeCreatorSet};

use crate::nodes::{ PBRCameraNode, PBRLightNode};

pub fn create_pbr_plugin() -> RenderScriptPlugin {
    let mut node_creators = NodeCreatorSet::default();
    node_creators.add::<PBRCameraNode>("PBR_CAMERA_EX".into());
    node_creators.add::<PBRLightNode>("PBR_LIGHT".into());
    RenderScriptPlugin::new(node_creators)
}