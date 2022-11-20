mod shadow;
mod shadow_light;
mod shadow_node;
use bevy_ecs::prelude::*;
use lite_clojure_eval::Variable;
use seija_app::App;
use seija_core::CoreStage;
pub use shadow_light::{ShadowLight};
pub use shadow::{Shadow};
pub use shadow_node::ShadowNode;
use crate::frp_context::FRPContext;
mod recv_backend;
#[derive(Component)]
pub struct ShadowCamera;


#[derive(Default)]
pub(crate) struct ShadowLights {
    count:u32,
    cache_enable:bool
}

pub(crate) fn shadow_frp_events(mut local_data:Local<ShadowLights>,add_shadows:Query<Entity,Added<ShadowLight>>,
                         remove_shadows:RemovedComponents<ShadowLight>,
                         frp_ctx:Res<FRPContext>) {
    for _ in add_shadows.iter() {
        local_data.count += 1;
    }
    for _ in remove_shadows.iter() {
        local_data.count -= 1;
    }
    
    let is_enable = local_data.count > 0;
    if is_enable != local_data.cache_enable {
        local_data.cache_enable = is_enable;
        let mut system = frp_ctx.inner.write();
        system.set_dynamic( "dynShadow".into(), Variable::Bool(is_enable));
    }
}

pub(crate) fn init_shadow(app:&mut App) {
    app.add_system(CoreStage::PostUpdate, shadow_frp_events);
}