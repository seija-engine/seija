use std::{borrow::Cow, sync::Arc};

use crate::resource::RenderResources;

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
            resources:RenderResources::new(arc_device)
        }
    }

    pub fn update(&mut self) {
        self.resources.next_swap_chain_texture();


        self.resources.clear_swap_chain_texture();
    }
}
