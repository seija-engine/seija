use bevy_ecs::prelude::Component;
use smol_str::SmolStr;

#[derive(Component,Debug)]
pub struct EInfo {
    pub name:Option<SmolStr>,
    pub layer:u32,
    pub tag:Option<SmolStr>
}
impl Default for EInfo {
    fn default() -> Self {
        Self { name: None, layer: 1, tag: None }
    }
}