use std::hash::{Hash, Hasher};

use bevy_ecs::prelude::Entity;
use fnv::FnvHasher;
use seija_core::{log, math::{Vec4, Mat4}};
use seija_render::material::Material;
use seija_transform::Transform;
use crate::{system::SystemParams, mesh2d::{Vertex2D, Mesh2D}};

pub struct RenderPanelInfo {
    pub(crate) panel_id:Entity,
    pub(crate) drawcalls:Vec<RenderDrawCall>
}


pub struct RenderDrawCall {
    pub key:u64,
    pub render_entity:Entity,
    pub sprites:Vec<Entity>
}

impl RenderDrawCall {
    pub fn create(panel_entity:Entity,params:&mut SystemParams,entitys:&Vec<Entity>) -> RenderDrawCall {
        let mut points:Vec<Vertex2D> = vec![];
        let mut indexs:Vec<u32> = vec![];
        let mut index_offset = 0u32;
        let mut hasher = FnvHasher::default();
        
        let parent_mat4 = params.calc_trans(panel_entity, None);

        for sprite_entity in entitys {
            sprite_entity.hash(&mut hasher);
            if let Ok((_,sprite,rect2d)) = params.sprites.get(*sprite_entity) {
                if let Some(sprite_index) = sprite.sprite_index {
                    if let Some((info,index)) = params.sprite_alloc.get_sprite_info(sprite_index) {
                        let mat4 = params.calc_trans(*sprite_entity, Some(panel_entity));
                        let zorder_value = params.zorders.get(*sprite_entity).map(|z| z.value).unwrap_or_default();
                        let mesh2d = sprite.build(rect2d,info.uv.clone(),index.atlas_index,&mat4,&info.rect,zorder_value as f32 * 0.01f32);


                        points.extend(mesh2d.points.iter());
                        indexs.extend(mesh2d.indexs.iter().map(|v| v + index_offset));
                        index_offset += mesh2d.points.len() as u32;

                    }
                }
            }
        }
        let key = hasher.finish();
        let mesh2d = Mesh2D {points,indexs,color:Vec4::ONE };
        let h_mesh = params.assets.0.add(mesh2d.into());
        let material = Material::from_def(params.render_data.baseui.clone(), &params.assets.2).unwrap();
        let h_material = params.assets.1.add(material);
        let transform = Transform::from_matrix(parent_mat4);
        let render_entity = params.commands.spawn((h_mesh,h_material,transform)).id();
       
        RenderDrawCall {
            key,
            render_entity,
            sprites:entitys.clone()
        }
    }
}