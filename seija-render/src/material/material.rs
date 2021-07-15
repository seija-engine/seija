use bevy_asset::Handle;
use bevy_render::{texture::{Texture}};
use bevy_reflect::TypeUuid;

#[derive(Debug)]
pub enum MaterialProp {
    Int(i32),
    Float(f32),
    Bool(bool),
    Texture(Option<Handle<Texture>>)
}

#[derive(Debug, TypeUuid)]
#[uuid = "506cff92-a6f2-4543-862d-6822c7fdfa99"]
pub struct MaterialDesc {
    pub props:Vec<(String,MaterialProp)>
}