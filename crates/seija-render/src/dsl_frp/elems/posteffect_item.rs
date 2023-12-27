use anyhow::Result;
use bevy_ecs::{prelude::Entity, world::World};
use lite_clojure_frp::FRPSystem;
use seija_asset::{Handle, AssetServer};
use smol_str::SmolStr;
use seija_core::OptionExt;

use crate::{dsl_frp::{frp_comp::IElement, system::ElementCreator, PostEffectStack}, RenderContext, material::Material};
pub struct PostEffectItem {
    camera_entity:Entity,
    material_path:SmolStr,
    sort_order:u32,

    material:Option<Handle<Material>>
}

impl PostEffectItem {
    pub fn new(entity:Entity,material_path:SmolStr,sort_order:u32) -> Result<PostEffectItem> {
        Ok(PostEffectItem {
            camera_entity:entity,
            material_path,
            sort_order,
            material:None
        })
    }
}

impl IElement for PostEffectItem {
    fn init(&mut self,world:&mut World,_:&mut RenderContext,
                _:&mut FRPSystem,_:&mut lite_clojure_eval::EvalRT,_:&ElementCreator) -> Result<()> {
        let asset_server = world.get_resource::<AssetServer>().get()?.clone();
        //let material = asset_server.load_sync::<Material>(world, &self.material_path, None)?;
        //self.material = Some(material);
        self.material = None;
        Ok(())
    }
    fn active(&mut self,world:&mut World,_:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        let mut camera_entity = world.get_entity_mut(self.camera_entity).get()?;
        if !camera_entity.contains::<PostEffectStack>() {
            camera_entity.insert(PostEffectStack::default());
        }
        let mut stack_mut = camera_entity.get_mut::<PostEffectStack>().get()?;
        let h_material = self.material.clone().get()?;
        stack_mut.add_item(h_material, self.sort_order);
        Ok(())
    }

    fn deactive(&mut self,world:&mut World,_:&mut RenderContext,_:&mut FRPSystem) -> Result<()> {
        let mut camera_entity = world.get_entity_mut(self.camera_entity).get()?;
        if camera_entity.contains::<PostEffectStack>() {
            let mut stack_mut = camera_entity.get_mut::<PostEffectStack>().get()?;
            if let Some(h_material) = self.material.as_ref() {
                stack_mut.remove_item_by_material(h_material);
            }
        }
        Ok(())
    }
}