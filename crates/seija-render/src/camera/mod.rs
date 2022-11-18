pub mod camera;
use std::{collections::HashMap, borrow::BorrowMut};

use lite_clojure_eval::Variable;
use lite_clojure_frp::DynamicID;
use seija_app::ecs::prelude::*;
use crate::frp_context::FRPContext;
use self::camera::Camera;

#[derive(Default)]
struct FRPCamera {
    cache_is_hdr:bool,
}


#[derive(Default)]
pub struct FRPCameras {
    camera:HashMap<Entity,FRPCamera>
}

pub fn camera_frp_event_system(mut local_data:Local<FRPCameras>,add_cameras:Query<(Entity,&Camera),Added<Camera>>,
                               remove_cameras:RemovedComponents<Camera>,
                               changed_cameras:Query<(Entity,&Camera),Changed<Camera>>,frp_ctx:Res<FRPContext>) {
    
    for (entity,camera) in add_cameras.iter() {
        local_data.camera.insert(entity, FRPCamera { cache_is_hdr:camera.is_hdr });
    }
    for rm_id in remove_cameras.iter() {
        local_data.camera.remove(&rm_id);
    }

    for (entity,camera) in changed_cameras.iter() {
        if let Some(frp_camera) = local_data.camera.get_mut(&entity) {
            if !frp_camera.cache_is_hdr != camera.is_hdr {
                frp_camera.cache_is_hdr = camera.is_hdr;
                let mut system = frp_ctx.inner.write();
                system.set_camera_dynamic(entity, ":dynIsHDR".into(), Variable::Bool(camera.is_hdr));
            }
        }
    }
}