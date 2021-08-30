use std::{borrow::Cow, sync::Arc};
use bevy_ecs::prelude::{Mut, World};

use crate::graph::{self, LinearGraphIter, RenderGraph};
use crate::resource::RenderResources;

#[derive(Default)]
pub struct RenderGraphContext {
    pub graph:RenderGraph,
    pub graph_iter:Arc<LinearGraphIter>,
}

impl RenderGraphContext {
    pub fn build_iter(&mut self) {
        self.graph_iter = Arc::new(LinearGraphIter::from_graph(&self.graph));
    }
}


pub struct AppRender {
    pub instance: wgpu::Instance,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub resources:RenderResources
}

pub struct Config {
    pub device_label:Option<Cow<'static,str>>,
    pub backed:wgpu::BackendBit,
    pub power_pref:wgpu::PowerPreference,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            device_label: None,
             backed: wgpu::BackendBit::VULKAN,
            power_pref: wgpu::PowerPreference::HighPerformance,
            features: Default::default(), 
            limits: Default::default()
        }
    }
}

impl AppRender {
    pub fn new_sync(config:Config) -> AppRender { futures_lite::future::block_on(Self::new(config)) }

    pub async fn new(config:Config) -> AppRender {
        let instance = wgpu::Instance::new(config.backed);
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference:config.power_pref,
            compatible_surface:None,
        }).await.expect("Unable to find a GPU!");
        
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label:config.device_label.as_ref().map(|a| a.as_ref()),
            features:config.features,
            limits:config.limits
        }, None).await.unwrap();
        let arc_device = Arc::new(device);
        AppRender {
            instance,
            device:arc_device.clone(),
            queue,
            resources:RenderResources::new(arc_device),
        }
    }

    pub fn update(&mut self,world:&mut World) {
        self.resources.next_swap_chain_texture();
        world.resource_scope(|world, mut render_ctx: Mut<RenderGraphContext>| {
            render_ctx.graph.prepare(world);
            for node_id in render_ctx.graph_iter.clone().nodes.iter() {
                let cur_node = render_ctx.graph.get_node(node_id).unwrap();
                let mut new_inputs = cur_node.inputs.clone();
                for parent_edge in cur_node.edges.input_edges.iter() {
                    let parent_node = render_ctx.graph.get_node(&parent_edge.output_node).unwrap();
                    for i in 0..parent_edge.output_idxs.len() {
                        let out_value = &parent_node.outputs[i];
                        new_inputs[parent_edge.input_idxs[i]] = out_value.clone();
                    }
                }
                 
                if let Ok(node) = render_ctx.graph.get_node_mut(node_id) {
                    
                    node.node.update(world, &new_inputs, &mut node.outputs);
                }
            }
        });
      
        self.resources.clear_swap_chain_texture();
    }
}
