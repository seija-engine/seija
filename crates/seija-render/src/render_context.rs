use std::sync::Arc;

use wgpu::{CommandEncoder, Device};

use crate::{camera::camera::CameraState, resource::RenderResources};

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}
pub struct RenderContext {
    pub device:Arc<Device>,
    pub resources:RenderResources,
    pub command_encoder:Option<CommandEncoder>,
    pub camera_state:CameraState
}