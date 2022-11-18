pub mod camera;
use std::{collections::HashMap, borrow::BorrowMut};

use lite_clojure_eval::Variable;
use lite_clojure_frp::DynamicID;
use seija_app::ecs::prelude::*;
use crate::{frp_context::FRPContext, dsl_frp::PostEffectStack};
use self::camera::Camera;

#[derive(Default)]
struct FRPCamera {
    cache_is_hdr:bool,
    cache_has_effect:bool
}


#[derive(Default)]
pub struct FRPCameras {
    camera:HashMap<Entity,FRPCamera>
}

pub fn camera_frp_event_system(mut local_data:Local<FRPCameras>,add_cameras:Query<(Entity,&Camera,Option<&PostEffectStack>),Added<Camera>>,
                               remove_cameras:RemovedComponents<Camera>,
                               changed_cameras:Query<(Entity,&Camera),Changed<Camera>>,
                               changed_stacks :Query<(Entity,&Camera,&PostEffectStack),Changed<PostEffectStack>>,frp_ctx:Res<FRPContext>) {
    
    for (entity,camera,stack) in add_cameras.iter() {
        let has_effect = stack.map(|v| v.items.len() > 0).unwrap_or(false);
        local_data.camera.insert(entity, FRPCamera { cache_is_hdr:camera.is_hdr,cache_has_effect:has_effect });

        let mut system = frp_ctx.inner.write();
        system.set_camera_dynamic(entity, ":dynIsHDR".into(), Variable::Bool(camera.is_hdr));
        system.set_camera_dynamic(entity, ":dynHasPostEffect".into(), Variable::Bool(has_effect));
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

    for (entity,_,stack) in changed_stacks.iter() {
        if let Some(frp_camera) = local_data.camera.get_mut(&entity) {
            let has_effect = stack.items.len() > 0;
            if !frp_camera.cache_has_effect != has_effect {
                frp_camera.cache_has_effect =has_effect;
                let mut system = frp_ctx.inner.write();
                system.set_camera_dynamic(entity, ":dynHasPostEffect".into(), Variable::Bool(has_effect));
            }
        }
    }
}