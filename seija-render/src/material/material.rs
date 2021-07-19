use std::{collections::HashMap, iter::FromIterator, sync::Arc};

use bevy_asset::Handle;
use bevy_render::{pipeline::PipelineDescriptor, prelude::RenderPipelines, texture::{Texture}};
use bevy_reflect::TypeUuid;
use glam::{Vec3, Vec4};

#[derive(Debug,Clone)]
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
    pub pipes:Vec<Handle<PipelineDescriptor>>
}

impl MaterialDesc {
    pub fn create(&self) -> Material {
        let pipes = RenderPipelines::from_handles(self.pipes.iter());
        let mut map:HashMap<String,MaterialProp> = HashMap::default();
        for (k,v) in self.props.iter() {
            map.insert(k.clone(), v.clone());
        }
        Material {
            props:map,
            pipes
        }
    }
}



#[derive(Debug, TypeUuid)]
#[uuid = "506cff92-a6f2-4543-862d-6822c7fdfa88"]
pub struct Material {
    pub props:HashMap<String,MaterialProp>,
    pub pipes:RenderPipelines
}

impl Material {
    pub fn set_textute(&mut self,name:&str,tex:Handle<Texture>) -> bool {
        if let Some(v) = self.props.get_mut(name) {
            *v = MaterialProp::Texture(Some(tex));
            return  true;
        }
        false
    }
}