use bevy_ecs::{system::{SystemParam, Query}, prelude::Entity, query::{Or, Changed}}; 
use super::{rect2d::Rect2D, input::Input};

#[derive(SystemParam)]
pub struct InputParams<'w,'s> {
    pub(crate) update_inputs:Query<'w,'s,Entity,Or<(Changed<Input>,Changed<Rect2D>)>>,
}

pub fn input_system(mut params:InputParams) {
  
}