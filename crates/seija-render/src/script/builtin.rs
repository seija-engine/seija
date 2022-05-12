use std::{collections::HashMap, convert::TryFrom};

use lite_clojure_eval::{Variable,GcRefCell};
use seija_core::LogOption;
use serde_json::Value;

use crate::{graph::{NodeId, nodes::{CameraCollect, SwapchainNode, PassNode, WindowTextureNode, TransformCollect, LightCollect, ScreenTextureNode}}, render::RenderGraphContext, material::{STextureDescriptor, RenderPath}};

use super::{NodeCreatorSet, NodeCreatorFn};

pub fn builtin_node_creators() -> NodeCreatorSet {
    let mut map:HashMap<String,NodeCreatorFn> = HashMap::default();
    map.insert("CAMERA".into()    , create_camera_node);
    map.insert("SWAP_CHAIN".into(), create_swap_chain_node);
    map.insert("PASS".into(), create_pass_node);
    map.insert("WINDOW_TEXTURE".into(), create_window_texture_node);
    map.insert("SCREEN_TEXTURE".into(), create_screen_texture_node);
    map.insert("TRANSFORM".into(), create_transform_node);
    map.insert("LIGHT".into(), create_light_node);
    NodeCreatorSet(map)
}

fn create_camera_node(ctx:&mut RenderGraphContext,params:Variable) -> NodeId {
    let map = params.cast_map().unwrap();
    let ubo_key = Variable::Keyword(GcRefCell::new(String::from(":ubo")));
    let ubo_name = map.borrow().get(&ubo_key).unwrap().cast_string().unwrap();
    let ubo_name_str:String = ubo_name.borrow().clone();

    let mut camera_collect = CameraCollect::default();
    camera_collect.ubo_name = ubo_name_str;
    ctx.graph.add_node("CameraCollect", camera_collect)
}

fn create_transform_node(ctx:&mut RenderGraphContext,params:Variable) -> NodeId {
    let map = params.cast_map().unwrap();
    let ubo_key = Variable::Keyword(GcRefCell::new(String::from(":ubo")));
    let ubo_name = map.borrow().get(&ubo_key).unwrap().cast_string().unwrap();
    let ubo_name_str:String = ubo_name.borrow().clone();

    let mut node = TransformCollect::default();
    node.ubo_name = ubo_name_str;
    ctx.graph.add_node("TransformCollect", node)
}

fn create_light_node(ctx:&mut RenderGraphContext,params:Variable) -> NodeId {
    let map = params.cast_map().unwrap();
    let ubo_key = Variable::Keyword(GcRefCell::new(String::from(":ubo")));
    let ubo_name = map.borrow().get(&ubo_key).unwrap().cast_string().unwrap();
    let ubo_name_str:String = ubo_name.borrow().clone();

    let mut light_node = LightCollect::default();
    light_node.ubo_name = ubo_name_str;
    ctx.graph.add_node("LightCollect", light_node)
}

fn create_swap_chain_node(ctx:&mut RenderGraphContext,params:Variable) -> NodeId {
    let swap_chain_node = SwapchainNode::new();
    ctx.graph.add_node("SwapChain", swap_chain_node)
}

fn create_pass_node(ctx:&mut RenderGraphContext,params:Variable) -> NodeId {
    let json_param:Value = params.into();
    let view_count = json_param.get(":view-count").and_then(Value::as_i64).unwrap_or(1) as usize;
    let is_depth = json_param.get(":is-depth").and_then(Value::as_bool).unwrap_or(true);
    let is_outinput = json_param.get(":is-outinput").and_then(Value::as_bool).unwrap_or(false);
  
    let str_path = json_param.get(":path").and_then(Value::as_str).unwrap_or("Foward");
    let path = RenderPath::try_from(str_path).unwrap_or(RenderPath::Forward);
    let pass_node = PassNode::new(view_count,is_depth,is_outinput,path);
    ctx.graph.add_node("PassNode", pass_node)
}

fn create_window_texture_node(ctx:&mut RenderGraphContext,params:Variable) -> NodeId {
    let window_texture_node = WindowTextureNode::new(wgpu::TextureDescriptor { 
        label: None,
        size: wgpu::Extent3d::default(),
        mip_level_count: 1,
        sample_count: 1, 
        dimension: wgpu::TextureDimension::D2, 
        format: wgpu::TextureFormat::Depth32Float, 
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT 
    });
    ctx.graph.add_node("WindowTexture", window_texture_node)
}

fn create_screen_texture_node(ctx:&mut RenderGraphContext,params:Variable) -> NodeId {
    let json_param:Value = params.into();
    let mut texture_descs:Vec<wgpu::TextureDescriptor> = vec![];
    if let Some(arr) = json_param.as_array().log_err("screen texture node need array param") {
        for item in arr.iter() {
            if let Ok(tex_desc) = STextureDescriptor::try_from(item) {
                texture_descs.push(tex_desc.0);
            } else {
                log::error!("into STextureDescriptor Error: {:?}",item);
            }
        }
    }
    let screen_texture_node = ScreenTextureNode::new(texture_descs);
    ctx.graph.add_node("ScreenTextureNode",screen_texture_node)
}