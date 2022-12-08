use bevy_ecs::prelude::Component;
use seija_app::{IModule, App};

pub mod sprite;
pub mod panel;
pub mod rect2d;


#[derive(Component)]
pub struct ElementTrack;