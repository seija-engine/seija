use std::collections::HashMap;

use smol_str::SmolStr;

use crate::{pipeline::render_bindings::BindGroupLayoutBuilder, uniforms::UniformTextureDef};

#[derive(Default,Debug)]
pub struct TexturePropDef {
    pub layout_builder:BindGroupLayoutBuilder,
    pub indexs:HashMap<String,TexturePropInfo>,
}

impl TexturePropDef {
    pub fn get_info(&self,name:&str) -> Option<&TexturePropInfo> {
        self.indexs.get(name)
    }
}

#[derive(Default,Debug)]
pub struct TexturePropInfo {
    pub name:String,
    pub index:usize,
    pub def_asset:SmolStr,
    pub is_cube_map:bool,
    pub texture_def:Option<UniformTextureDef>
}