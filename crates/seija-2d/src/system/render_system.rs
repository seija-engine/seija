use bevy_ecs::{system::{Query, Commands, Res, ResMut}, entity::Entity, query::ChangeTrackers};
use seija_asset::{Handle, AssetServer, Assets};
use seija_core::math::Mat4;
use seija_render::{material::Material, resource::Mesh};
use spritesheet::SpriteSheet;

use crate::{components::{image::Image, sprite::Sprite2D}, Module2DResource, common::Rect2D};


pub fn image_and_sprite_system(
                    update_images:Query<(Entity,&Image,&Rect2D,ChangeTrackers<Image>)>,
                    update_sprites:Query<(Entity,&Sprite2D,&Rect2D,ChangeTrackers<Sprite2D>)>,
                    mut commands:Commands,res2d:Res<Module2DResource>,
                    server:Res<AssetServer>,mut mats:ResMut<Assets<Material>>,
                    mut meshs:ResMut<Assets<Mesh>>,
                    mut sheets:ResMut<Assets<SpriteSheet>>) {
    for (entity,image,rect2d,tracker) in update_images.iter() {
        if tracker.is_added() {
            init_image(entity, &image,&rect2d,&mut commands,&res2d,&server,&mut mats,&mut meshs);
        }
    }
    for (entity,sprite2d,rect2d,tracker) in update_sprites.iter() {
        if tracker.is_added() {
            init_sprite2d(entity, sprite2d, rect2d, &mut commands,&mut sheets,&res2d,&server,&mut mats,&mut meshs);
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

fn init_sprite2d(entity:Entity,sprite2d:&Sprite2D,rect2d:&Rect2D,commands:&mut Commands,sheets:&mut Assets<SpriteSheet>,
    res2d:&Module2DResource,server:&AssetServer,mats:&mut Assets<Material>,meshs:&mut Assets<Mesh>) {
    
    let cur_sheet = sprite2d.sheet.as_ref().and_then(|id| sheets.get(&id.id));
    let mut h_material:Option<Handle<Material>> = if let Some(material) = sprite2d.custom_material.as_ref() {
        Some(material.clone())
    } else {
        if let Some(h_texture) = cur_sheet.map(|s| &s.texture) {
            let mut material = Material::from_def(res2d.image_material_define.clone(),server).unwrap();
            material.texture_props.set("mainTexture", h_texture.clone());
            let new_material = mats.add(material);
            Some(new_material)
        } else { None }
    };
    /*
    if let Some(info) = cur_sheet.and_then(|s| s.get_info(sprite2d.sprite_index)) {
        let mesh2d = sprite2d.common.build_mesh(&Mat4::IDENTITY, rect2d, &info.uv, &info.rect, 0f32);
        let mesh:Mesh = mesh2d.into();
        let h_mesh = meshs.add(mesh);
        if let Some(h_material) = h_material.take() {
            let mut entity_mut = commands.entity(entity);
            entity_mut.insert(h_material).insert(h_mesh);
        }
    }*/
}