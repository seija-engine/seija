use bevy_ecs::prelude::Component;
use smol_str::SmolStr;

#[derive(Component,Default,Debug)]
pub struct EInfo {
    pub name:Option<String>,
    pub layer:u32,
    pub tag:Option<SmolStr>
}