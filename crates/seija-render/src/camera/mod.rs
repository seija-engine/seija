pub mod camera;
use std::collections::HashMap;

use lite_clojure_frp::DynamicID;
use seija_app::ecs::prelude::*;
use crate::frp_context::FRPContext;
use self::camera::Camera;

struct FRPCamera {
    dyn_is_hdr:DynamicID
}

struct FRPCameras {
    camera:HashMap<Entity,FRPCamera>
}

pub fn camera_event_system(add_cameras:Query<(Entity,&Camera),Added<Camera>>,frp_ctx:Res<FRPContext>) {
    for (entity,camera) in add_cameras.iter() {
      
    }
}