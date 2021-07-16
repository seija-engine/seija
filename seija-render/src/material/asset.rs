use std::{collections::HashMap, convert::TryInto, path::Path, sync::Arc};

use bevy_asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy_render::{pipeline::{ColorTargetState, PipelineDescriptor}, shader::{Shader, ShaderStage, ShaderStages}};
use glam::{Vec3, Vec4};
use lite_clojure_eval::{EvalRT,Variable};
use anyhow::Result;
use serde_json::Value;

use crate::material::{json_der::MValue, material::{MaterialProp,MaterialDesc}};

#[derive(Default)]
pub struct MaterialDescLoader;


impl AssetLoader for MaterialDescLoader {
    fn load<'a>(&'a self, bytes: &'a [u8], load_context: &'a mut LoadContext) -> BoxedFuture<Result<()>> {
       
        Box::pin(async move {
           let json_value = {
                let mut eval = EvalRT::new();
                eval.init();
                let utf8_str = std::str::from_utf8(bytes)?;
                eval.eval_string(String::default(), utf8_str).map(|v|v.into())
            };
            if let Some(v) = json_value {
                let desc = load_meterial_desc(v,load_context).await?;
               
                let s = load_context.path().to_str().unwrap().to_string();
                load_context.set_labeled_asset(s.as_str(), LoadedAsset::new(desc));
                Ok(())
            } else {
                anyhow::bail!("err MaterialDesc")
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["material"]
    }
}

async fn load_meterial_desc<'a,'b>(var:Value, load_context: &mut LoadContext<'b>) -> anyhow::Result<MaterialDesc> {
    let attr_map = var.as_object().ok_or(anyhow::anyhow!("err MaterialDesc"))?;
    let mut props:Vec<(String,MaterialProp)> = vec![];
    let mut pipes:Vec<PipelineDescriptor> = vec![];
    for (k,v) in attr_map {
        match k.as_str() {
            ":props" => {
                props = load_matrial_props(v)?;
            }
            ":pass" => {
                pipes = load_material_pass(v,load_context).await?;
            }
            _ => {}
        }
    }
    Ok(MaterialDesc {
        props,
        pipes
    })
}

fn load_matrial_props(var:&Value) -> anyhow::Result<Vec<(String,MaterialProp)>> {
    let props_map = var.as_object().ok_or(anyhow::anyhow!("err MaterialDesc"))?;
    let mut arr :Vec<(String,MaterialProp)>= vec![];
    for (k,v) in props_map {
        let k_str = k.clone();
        let ty = v.as_str().ok_or(anyhow::anyhow!("err material props type"))?;
        match ty {
            ":Int" => arr.push((k_str,MaterialProp::Int(0))),
            ":Float" => arr.push((k_str,MaterialProp::Float(0f32))),
            ":Texture" => arr.push((k_str,MaterialProp::Texture(None))),
            ":Bool" => arr.push((k_str,MaterialProp::Bool(false))),
            ":Vec3" => arr.push((k_str,MaterialProp::Vector3(Vec3::ZERO))),
            ":Vec4" => arr.push((k_str,MaterialProp::Vector4(Vec4::ZERO))),
            _ => anyhow::bail!("error props type")
        }
    }
    Ok(arr)
}

async fn load_material_pass<'a,'b>(var:&Value,load_context: &mut LoadContext<'b>) -> Result<Vec<PipelineDescriptor>> {
    let mut pass_list:Vec<PipelineDescriptor> = vec![];
    match var {
        Value::Array(lst) => {
            for item in lst {
                let desc = load_material_single_pass(item,load_context).await?;
                pass_list.push(desc);
            }
        },
        Value::Object(_) => {
            pass_list.push(load_material_single_pass(var,load_context).await?);
        }
       _ => ()
    }

    Ok(pass_list)
}

async fn load_material_single_pass<'a,'b>(var:&'a Value,load_context: &mut LoadContext<'b>) -> Result<PipelineDescriptor> {
    let map = var.as_object().ok_or(anyhow::anyhow!("error pass desc"))?;
    let vert_shader_path = map.get(":shader-vert").unwrap().as_str().unwrap();
    let vert_bytes = load_context.read_asset_bytes(vert_shader_path).await?;
    let vert_shader = Shader::from_glsl(ShaderStage::Vertex, std::str::from_utf8(&vert_bytes)?);
    let vert_shader_handle = load_context.set_labeled_asset(vert_shader_path, LoadedAsset::new(vert_shader));

    let frag_shader_path = map.get(":shader-frag").unwrap().as_str().unwrap();
    let frag_bytes = load_context.read_asset_bytes(frag_shader_path).await?;
    let frag_shader = Shader::from_glsl(ShaderStage::Fragment, std::str::from_utf8(&frag_bytes)?);
    let frag_shader_handle = load_context.set_labeled_asset(frag_shader_path, LoadedAsset::new(frag_shader));
    let shader_stages = ShaderStages {  vertex: vert_shader_handle, fragment: Some(frag_shader_handle), };
    let mut pipeline_desc = PipelineDescriptor::new(shader_stages);
    for (k,v) in map {
        match k.as_str() {
             ":name" => {
                 pipeline_desc.name = Some(v.as_str().unwrap().to_string())
             },
             ":primitive" => {
                 let p = MValue(v.clone()).try_into().map_err(|_| anyhow::anyhow!("err primitive"))?;
                 pipeline_desc.primitive = p;
             },
             ":depth-stencil" => {
                 let s = MValue(v.clone()).try_into().map_err(|_| anyhow::anyhow!("err depth-stencil"))?;
                 pipeline_desc.depth_stencil = Some(s)
             }
             ":multisample" => {
                 let v = MValue(v.clone()).try_into().map_err(|_| anyhow::anyhow!("err multisample"))?;
                 pipeline_desc.multisample = v;
             },
             ":color-target-states" => {
                 let arr = v.as_array().ok_or( anyhow::anyhow!("err color-target-states"))?;
                 let mut states:Vec<ColorTargetState> = vec![];
                 for item in arr {
                     states.push(MValue(item.clone()).try_into().map_err(|_| anyhow::anyhow!("err color-target-states"))?);
                 }
                 pipeline_desc.color_target_states = states;
             }
             _ => {}
        }
    }
    Ok(pipeline_desc)
}