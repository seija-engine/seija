use std::collections::HashMap;

use crate::pipeline::render_bindings::BindGroupLayoutBuilder;

#[derive(Default,Debug)]
pub struct TexturePropDef {
    pub layout_builder:BindGroupLayoutBuilder,
    pub indexs:HashMap<String,(usize,usize)>,
}