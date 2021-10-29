use std::sync::Arc;

use wgpu::{CommandEncoder, Device};

use crate::{TransformBuffer, camera::system::CameraState, material::MaterialSystem, resource::RenderResources};

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}
pub struct RenderContext {
    pub device:Arc<Device>,
    pub resources:RenderResources,
    pub command_encoder:Option<CommandEncoder>,
    pub camera_state:CameraState,
    pub transform_buffer:TransformBuffer,
    pub material_sys:MaterialSystem,
}