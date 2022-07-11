use bevy_ecs::prelude::Component;

use crate::exposure::Exposure;

#[derive(Default,Component)]
pub struct PBRCameraInfo {
    pub exposure:Exposure
}
