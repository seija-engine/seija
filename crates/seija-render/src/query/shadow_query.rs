use bevy_ecs::prelude::{Query, Entity, Res, With};
use seija_asset::Handle;

use crate::{shadow::Shadow, material::{Material, RenderOrder}, resource::Mesh};

use super::{ViewQuery, QuerySystem, view_list::ViewEntity};

const SHADOW_TYPE:u32 = 2u32;

pub(crate) fn shadow_query_update(system:Res<QuerySystem>,shadow_query:Query<(Entity,&Shadow),(With<Handle<Material>>,With<Handle<Mesh>>)>) {
    for view_query in system.querys.iter() {
        if view_query.read().typ == SHADOW_TYPE {
            update_shadow_query(&mut view_query.write(),&shadow_query);
        }
   }
}

fn update_shadow_query(view_query:&mut ViewQuery,shadows:&Query<(Entity,&Shadow),(With<Handle<Material>>,With<Handle<Mesh>>)>) -> Option<()> {
    view_query.list.clear();
    for (e,s) in shadows.iter() {
        if s.cast_shadow {
            view_query.list.add_entity(RenderOrder::Opaque, ViewEntity {entity:e,order:0f32  })
        }
    }
    None
}