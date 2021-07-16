use bevy_asset::Handle;
use bevy_render::{pipeline::PipelineDescriptor, texture::{Texture}};
use bevy_reflect::TypeUuid;
use glam::{Vec3, Vec4};

#[derive(Debug)]
pub enum MaterialProp {
    Int(i32),
    Float(f32),
    Bool(bool),
    Vector3(Vec3),
    Vector4(Vec4),
    Texture(Option<Handle<Texture>>)
}

#[derive(Debug, TypeUuid)]
#[uuid = "506cff92-a6f2-4543-862d-6822c7fdfa99"]
pub struct MaterialDesc {
    pub props:Vec<(String,MaterialProp)>,
    pub pipes:Vec<PipelineDescriptor>
}