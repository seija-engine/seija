use std::collections::HashMap;

use bevy_ecs::prelude::Component;
use bevy_ecs::query::ChangeTrackers;
use bevy_ecs::system::{Res, Resource, ResMut};
use bevy_ecs::{system::{SystemParam, Query}, prelude::Entity, query::{Or, Changed}}; 
use crate::text::Text;

use super::rect2d::Rect2D;
use seija_input::Input as SysInput;
#[derive(Component,Debug,Clone,Default)]
#[repr(C)]
pub struct Input {
    pub is_focus:bool,
    pub text:String
}

#[derive(Resource,Default)]
pub struct InputTextSystemData {
    cache_dict:HashMap<Entity,InputTextCache>
}

pub struct InputTextCache {
    pub entity:Entity
}

impl InputTextCache {
    pub fn new(entity:Entity) -> InputTextCache {
        InputTextCache { entity }
    }
}

#[derive(SystemParam)]
pub struct InputParams<'w,'s> {
    pub(crate) update_inputs:Query<'w,'s,(Entity,&'static Input),Or<(Changed<Input>,Changed<Rect2D>)>>,
    pub(crate) input:Res<'w,SysInput>,
    pub(crate) texts:Query<'w,'s,&'static mut Text>,
    pub(crate) sys_data:ResMut<'w,InputTextSystemData>
}

pub fn input_system(mut params:InputParams) {
   for v in params.update_inputs.iter() {
    //check init input cache
    if !params.sys_data.cache_dict.contains_key(&v.0) {
       let text_cache = InputTextCache::new(v.0);
       params.sys_data.cache_dict.insert(v.0, text_cache);
    }
    
    if let Ok(mut text) = params.texts.get_mut(v.0) {
        text.text = v.1.text.clone();
    }
   }
}