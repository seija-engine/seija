use std::{convert::{TryFrom}, sync::{Arc}};
use seija_core::{TypeUuid};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
use smol_str::SmolStr;
use wgpu::{FrontFace, PolygonMode};
use super::{RenderOrder, errors::MaterialDefReadError, texture_prop_def::TexturePropDef, types::{Cull, SFrontFace, SPolygonMode, ZTest, RenderPath, STextureFormat}, TexturePropInfo};
use lite_clojure_eval::EvalRT;
use serde_json::{Value};
use uuid::Uuid;
use crate::{memory::UniformBufferDef};

#[derive(Debug)]
pub struct MaterialDef {
    pub name:SmolStr,
    pub path:RenderPath,
    pub order:RenderOrder,
    pub pass_list:Vec<PassDef>,
    pub prop_def:Arc<UniformBufferDef>,
    pub tex_prop_def:Arc<TexturePropDef>,
}

#[derive(Debug,TypeUuid)]
#[uuid = "58ee0320-a01e-4a1b-9d07-ade19767853b"]
pub struct MaterialDefineAsset {
    pub define:Arc<MaterialDef>
}

#[derive(Debug)]
pub struct PassDef {
    pub conservative:bool,
    pub polygon_mode:SPolygonMode,
    pub front_face:SFrontFace,
    pub z_write:bool,
    pub z_test:ZTest,
    pub cull:Cull,
    pub clamp_depth:bool,
    pub shader_info:ShaderInfoDef,
    pub targets:Vec<TargetInfo>,
    pub tag:Option<String>
   
}

#[derive(Debug)]
pub struct TargetInfo {
    format:wgpu::TextureFormat,
    blend:Option<wgpu::BlendState>,
    write_mask:wgpu::ColorWrite
}

impl Default for PassDef {
    fn default() -> Self {
        Self {
            conservative:false,
            polygon_mode:SPolygonMode(PolygonMode::Fill),
            front_face:SFrontFace(FrontFace::Ccw),
            z_write:true,
            z_test:ZTest::Less,
            clamp_depth:false,
            cull: Cull::Back,
            shader_info:ShaderInfoDef::default(),
            targets:vec![],
            tag:None
        }
    }
}


pub fn read_material_def(vm:&mut EvalRT,file_string:&str,read_slot:bool) -> Result<MaterialDef,MaterialDefReadError>  {
    let value:Value = vm.eval_string(String::default(), file_string).ok_or(MaterialDefReadError::LanguageError)?.into();
    let value_object = value.as_object().ok_or(MaterialDefReadError::FormatError)?;
    
    //name
    let def_name = value_object.get(":name").and_then(|v| v.as_str()).ok_or(MaterialDefReadError::InvalidName)?;
    //order
    let order_str = value.get(":order").and_then(|v| v.as_str()).unwrap_or(&"Opaque");
    let order = RenderOrder::try_from(order_str).map_err(|s| MaterialDefReadError::InvalidOrder(s))?;
    //path
    let path_str = value.get(":path").and_then(|v| v.as_str()).unwrap_or(&"Forward");
    let path = RenderPath::try_from(path_str).map_err(|s| MaterialDefReadError::InvalidRenderPath(s))?;
    //pass
    let json_pass = value.get(":pass").ok_or(MaterialDefReadError::InvalidPass)?;
    let mut pass_list:Vec<PassDef> = Vec::new();
    match json_pass {
        Value::Array(arr) => {
            for v in arr {
                pass_list.push(read_pass(v,read_slot)?);
            }
        },
        Value::Object(_) => { pass_list.push(read_pass(json_pass,read_slot)?); },
        _ => return Err(MaterialDefReadError::InvalidPass)
    }

    let prop_value = value.get(":props").ok_or(MaterialDefReadError::InvalidProp)?;
    let buffer_def = UniformBufferDef::try_from(prop_value).map_err(|_| MaterialDefReadError::InvalidProp)?;
    let texture_prop_def = read_texture_prop(prop_value).map_err(|_| MaterialDefReadError::InvalidProp)?;
    Ok(MaterialDef {
        name:def_name.into(),
        path,
        order,
        pass_list,
        prop_def:Arc::new(buffer_def),
        tex_prop_def:Arc::new(texture_prop_def)
    })
}

fn read_texture_prop(json_value:&Value) -> Result<TexturePropDef,()> {
    let arr = json_value.as_array().ok_or( ())?;
    let mut texture_props = TexturePropDef::default();
   
    let mut texture_index:usize = 0;
    for item in arr {
        if let Some(map) = item.as_object() {
            let prop_type = map.get(":type").and_then(|v| v.as_str()).ok_or(())?;
            let prop_name = map.get(":name").and_then(|v| v.as_str()).ok_or(())?;
            let def_name = map.get(":default").and_then(|v| v.as_str());
            let mut def_asset = SmolStr::new("texture:white");
            if let Some(def_name) = def_name {
                let mut s = String::from("texture:");
                s.push_str(def_name);
                def_asset = s.into();
            }
            let mut texture_prop = TexturePropInfo {
                name:prop_name.to_string(),
                index:texture_index,
                def_asset,
                is_cube_map:false
            };
            match prop_type {
                "Texture" => {
                    //TODO 这里需要根据贴图变化布局
                    texture_props.layout_builder.add_texture(false,None);
                    texture_props.layout_builder.add_sampler(true);
                    texture_prop.is_cube_map = false;
                    texture_props.indexs.insert(prop_name.to_string(), texture_prop);
                   
                    texture_index += 1;
                },
                "CubeMap" => {
                    texture_props.layout_builder.add_texture(true,None);
                    texture_props.layout_builder.add_sampler(true);
                    texture_prop.is_cube_map = true;
                    texture_props.indexs.insert(prop_name.to_string(), texture_prop);
                    texture_index += 1;
                },
                _ => {}
            }
        }
    }
    Ok(texture_props)
}

fn read_pass(json_value:&Value,read_slot:bool) -> Result<PassDef,MaterialDefReadError> {
    let map = json_value.as_object().ok_or(MaterialDefReadError::InvalidPass)?;
    let mut pass_def = PassDef::default();
    if let Some(z_write) = map.get(":z-write").and_then(|v| v.as_bool()) {
        pass_def.z_write = z_write;
    }
    if let Some(s) = map.get(":front-face").and_then(|v| v.as_str()) {
        pass_def.front_face = SFrontFace::try_from(s).map_err(|_| MaterialDefReadError::InvalidPassProp(":front-face".into()) )?
    }
    //if let Some(b) = map.get(":clamp-depth").and_then(|v| v.as_bool()) {
    //    pass_def.clamp_depth = b;
    //}
    if let Some(tag) = map.get(":tag").and_then(|v| v.as_str()) {
        pass_def.tag = Some(tag.into());
    }
    if let Some(z_test) = map.get(":z-test").and_then(|v| v.as_str()) {
        pass_def.z_test = ZTest::try_from(z_test).map_err(|_| MaterialDefReadError::InvalidPassProp(":z-test".into()))?;
    }
    if let Some(cull) = map.get(":cull").and_then(|v| v.as_str()) {
        pass_def.cull = Cull::try_from(cull).map_err(|_| MaterialDefReadError::InvalidPassProp(":cull".into()))?;
    }
    if let Some(poly) = map.get(":polygon-mode").and_then(|v| v.as_str()) {
        pass_def.polygon_mode = SPolygonMode::try_from(poly).map_err(|_| MaterialDefReadError::InvalidPassProp(":polygon-mode".into()))?;
    }
    if let Some(b) = map.get(":conservative").and_then(|v| v.as_bool()) {
        pass_def.conservative = b;
    }
    let shader_value =  map.get(":shader").ok_or(MaterialDefReadError::InvalidPassProp("shader".into()))?;
    pass_def.shader_info = read_shader_info(shader_value,read_slot)?;
    let mut targets:Vec<TargetInfo> = vec![];
    if let Some(json_targets) = map.get(":targets").and_then(Value::as_array) {
        for json_target in json_targets.iter() {
           if let Ok(target_info) = TargetInfo::try_from(json_target) {
               targets.push(target_info);
           } else {
               log::error!("load :targets error:{:?}",json_target);
           }
        }
    } else {
        targets.push(TargetInfo::default());
    }
    pass_def.targets = targets;
    Ok(pass_def)
}

#[derive(Debug,Default)]
pub struct ShaderInfoDef {
    pub name:String,
    pub slots:Option<String>,
    pub features:Vec<SmolStr>
}


fn read_shader_info(value:&Value,read_slot:bool) -> Result<ShaderInfoDef,MaterialDefReadError> {
    let object_value = value.as_object().ok_or(MaterialDefReadError::InvalidPassProp("shader".into()))?;
    let name = object_value.get(":name").and_then(|v| v.as_str()).ok_or(MaterialDefReadError::InvalidPassProp("shader".into()))?;
    let mut features = vec![];
    if let Some(json_features) = object_value.get(":features") {
        features = json_features.as_array().map(|arr| {
           arr.iter().filter_map(|v| v.as_str()).map(SmolStr::new).collect()
       }).unwrap_or(vec![]);
    }
    let mut slots = None;
    if read_slot {
        if let Some(slot_string) = object_value.get(":slot").and_then(Value::as_str) {
            slots = Some(slot_string.to_string())
        }
    } else {
        if let Some(slot_string) = object_value.get(":slot").and_then(Value::as_str) {
            let mut hasher = DefaultHasher::default();
            slot_string.hash(&mut hasher);
            hasher.finish();
            slots = Some(hasher.finish().to_string());
        }
    }
    Ok(ShaderInfoDef {
        name:name.to_string(),
        slots,
        features
    })
}

impl PassDef {
    pub fn get_color_targets(&self) -> Vec<wgpu::ColorTargetState> {
        let mut color_targets:Vec<wgpu::ColorTargetState> = vec![];
        for target in self.targets.iter() {
            let target = wgpu::ColorTargetState {
                format: target.format.clone(),
                blend: target.blend.clone(),
                write_mask: target.write_mask.clone(),
            };
            color_targets.push(target);
        }
        color_targets
    }
}

impl TryFrom<&Value> for TargetInfo {
    type Error = ();
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let mut default_target = TargetInfo::default();

        let value_map = value.as_object().ok_or(())?;
        let str_format = value_map.get(":format").and_then(Value::as_str).ok_or(())?;
        let sformat = STextureFormat::try_from(str_format)?;
        
        default_target.format = sformat.0;
        if let Some(blend) = value_map.get(":blend") {
            if blend.is_null() {
                default_target.blend = None;
            } else {
                //TODO
                
                todo!()
            }
        }
        Ok(default_target)
    }
}

impl Default for TargetInfo {
    fn default() -> Self {
        Self { 
             format: wgpu::TextureFormat::Bgra8UnormSrgb,
             blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
            }), 
             write_mask: wgpu::ColorWrite::ALL
        }
    }
}