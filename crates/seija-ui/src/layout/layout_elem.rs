use bevy_ecs::prelude::Component;

use crate::types::Thickness;

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
pub enum LayoutAlignment {
    Start = 0,
	Center = 1,
	End = 2,
	Fill = 3
}

#[derive(Component)]
pub struct LayoutElement {
    pub size:Vec<f32>,
    pub margin: Thickness,
    pub padding: Thickness,
    pub hor: LayoutAlignment,
    pub ver: LayoutAlignment,
}