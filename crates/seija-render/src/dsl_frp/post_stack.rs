use bevy_ecs::{prelude::Component, system::{ResMut, Query, Res}};
use seija_asset::{Handle, Assets};

use crate::material::{Material, GlobalImportMaterials};


pub struct EffectItem {
    pub material:Handle<Material>,
    pub(crate) order:u32  
}

#[derive(Component,Default)]
pub struct PostEffectStack {
    pub items:Vec<EffectItem>,
    is_dirty:bool
}

impl PostEffectStack {
    pub fn add_item(&mut self,material:Handle<Material>,order:u32) {
        self.items.push(EffectItem { material,order});
        self.items.sort_by(|a,b|a.order.cmp(&b.order));
        self.is_dirty = true;
    }

    pub fn remove_item_by_material(&mut self,handle:&Handle<Material>) {
        for idx in (self.items.len() .. 0).rev() {
            let item = &self.items[idx];
            if item.material.id == handle.id {
                self.items.remove(idx);
                return;
            }
        } 
    }
}

/*
pub fn post_material_to_global(mut posteffects:Query<&mut PostEffectStack>,materials:Res<Assets<Material>>,mut global_material:ResMut<GlobalImportMaterials>) {
    for mut effect in posteffects.iter_mut() {
        if effect.is_dirty {
            for item in effect.items.iter() {
                if let Some(material) = materials.get(&item.material.id) {
                    global_material.add(&material.def);
                }
            }
            effect.is_dirty = false;
        }
    }
}*/

