use bevy_ecs::{system::{Query, Commands, Res, ResMut}, entity::Entity, query::{ChangeTrackers, Changed}};
use seija_asset::{Handle, AssetServer, Assets};
use seija_core::{math::Vec3, Rect, info::EStateInfo};
use seija_render::{material::Material, resource::Mesh};
use spritesheet::SpriteSheet;

use crate::{components::{image::Image, sprite::Sprite2D}, Module2DResource, common::Rect2D};


pub fn image_and_sprite_system(
                    update_images:Query<(Entity,&Image,&Rect2D,ChangeTrackers<Image>)>,
                    update_sprites:Query<(Entity,&Sprite2D,&Rect2D,Option<&EStateInfo>,ChangeTrackers<Sprite2D>,ChangeTrackers<Rect2D>)>,
                    hmat_query:Query<&Handle<Material>>,
                    mut commands:Commands,res2d:Res<Module2DResource>,
                    server:Res<AssetServer>,mut mats:ResMut<Assets<Material>>,
                    mut meshs:ResMut<Assets<Mesh>>,
                    sheets:Res<Assets<SpriteSheet>>,
                    active_changed:Query<Entity,Changed<EStateInfo>>) {
    for (entity,image,rect2d,tracker) in update_images.iter() {
        if tracker.is_added() {
            init_image(entity, &image,&rect2d,&mut commands,&res2d,&server,&mut mats,&mut meshs);
        }
    }
    for (entity,sprite2d,rect2d,info,sprite_tracker,rect_tracker) in update_sprites.iter() {
        let is_active = info.map(|v| v.is_active_global()).unwrap_or(true);
        if sprite_tracker.is_added() {
            init_sprite2d(entity, sprite2d, rect2d, &mut commands,&sheets,&res2d,&server,&mut mats,&mut meshs);
        }
        if sprite_tracker.is_changed() && is_active {
            update_sprite2d(entity,sprite2d,&mut mats,&hmat_query,&sheets);
        }
        if (rect_tracker.is_changed() && !rect_tracker.is_added() && is_active) || active_changed.contains(entity) {
            let mesh2d = sprite2d.build_mesh(rect2d);
            let mesh:Mesh = mesh2d.into();
            let h_mesh = meshs.add(mesh);
            commands.entity(entity).remove::<Handle<Mesh>>().insert(h_mesh);
        }
    }
}

fn init_image(entity:Entity,image:&Image,rect2d:&Rect2D,commands:&mut Commands,
    res2d:&Module2DResource,server:&AssetServer,mats:&mut Assets<Material>,meshs:&mut Assets<Mesh>) {
    let mut entity_mut = commands.entity(entity);
    let h_material:Handle<Material> = if let Some(material) = image.custom_material.as_ref() {
        material.clone()  
    } else {
        let mut material = Material::from_def(res2d.image_material_define.clone(),server).unwrap();
        material.texture_props.set("mainTexture", image.texture.clone());
        let new_material = mats.add(material);
        new_material
    };
    let mesh2d = image.build_mesh(rect2d);
    let mesh:Mesh = mesh2d.into();
    let h_mesh = meshs.add(mesh);
    entity_mut.insert(h_material).insert(h_mesh);
}

fn init_sprite2d(entity:Entity,sprite2d:&Sprite2D,rect2d:&Rect2D,commands:&mut Commands,sheets:&Assets<SpriteSheet>,
    res2d:&Module2DResource,server:&AssetServer,mats:&mut Assets<Material>,meshs:&mut Assets<Mesh>) {
    
    let cur_sheet = sprite2d.sheet.as_ref().and_then(|id| sheets.get(&id.id));
    let mut h_material:Option<Handle<Material>> = if let Some(material) = sprite2d.custom_material.as_ref() {
        Some(material.clone())
    } else {
        if let Some(h_texture) = cur_sheet.map(|s| &s.texture) {
            let mut material = Material::from_def(res2d.sprite_material_define.clone(),server).unwrap();
            material.texture_props.set("mainTexture", h_texture.clone());
            if let Some(info) = cur_sheet.and_then(|s| s.get_info(sprite2d.sprite_index)) {
                set_sprite_uv(&mut material, &info.uv);
            }
            let new_material = mats.add(material);
            Some(new_material)
        } else { None }
    };
    
    let mesh2d = sprite2d.build_mesh(rect2d);
    let mesh:Mesh = mesh2d.into();
    let h_mesh = meshs.add(mesh);
    if let Some(h_material) = h_material.take() {
        let mut entity_mut = commands.entity(entity);
        entity_mut.insert(h_material).insert(h_mesh);
    }
}

fn update_sprite2d(entity:Entity,sprite2d:&Sprite2D,mats:&mut Assets<Material>,hmat_query:&Query<&Handle<Material>>,sheets:&Assets<SpriteSheet>) {
    if sprite2d.is_material_dirty() {
        if let Ok(h_mat) = hmat_query.get(entity) {
            if let Some(material) = mats.get_mut(&h_mat.id) {
                material.props.set_float4("color", sprite2d.color, 0);
                let cur_sheet = sprite2d.sheet.as_ref().and_then(|id| sheets.get(&id.id));
                if let Some(info) = cur_sheet.and_then(|s| s.get_info(sprite2d.sprite_index)) {
                    set_sprite_uv(material, &info.uv);
                }
                if sprite2d.is_sheet_dirty() {
                    if let Some(sheet) = cur_sheet {
                        material.texture_props.set("mainTexture", sheet.texture.clone());
                    }
                }
            }
        }
        sprite2d.clear_dirtys();
    }
}

fn set_sprite_uv(material:&mut Material,uv:&Rect<f32>) {
    material.props.set_float3("uvBuffer", Vec3::new(uv.x, uv.y, 0f32), 0);
    material.props.set_float3("uvBuffer", Vec3::new(uv.x + uv.width,uv.y, 0f32), 1);
    material.props.set_float3("uvBuffer", Vec3::new(uv.x,uv.y + uv.height, 0f32), 2);
    material.props.set_float3("uvBuffer", Vec3::new(uv.x + uv.width,uv.y + uv.height, 0f32), 3);
}