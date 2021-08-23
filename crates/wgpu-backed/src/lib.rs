use std::sync::Arc;

use seija_render::core::{IBackend,IRenderResourceContext};

pub mod resource_context;
mod type_converter;


#[derive(Clone)]
pub enum WGPUBackendType {
    Auto,
    Vulkan,
    Metal,
    Dx12,
    Dx11,
    Gl,
    BrowserWGPU,
}

impl Default for WGPUBackendType {
    fn default() -> Self { WGPUBackendType::Vulkan }
}

#[derive(Default, Clone)]
pub struct WGPUConfig {
    pub backend: WGPUBackendType,
}


pub struct WGPUBackend {
    pub instance: wgpu::Instance,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
}

impl Default for WGPUBackend {
    fn default() -> Self { WGPUBackend::new_symc(WGPUConfig::default()) }
}

impl WGPUBackend {
    pub fn new_symc(config:WGPUConfig) -> WGPUBackend {
        futures_lite::future::block_on(Self::new(config)) 
    }

    pub async fn new(config:WGPUConfig) -> WGPUBackend {
        let backend = match config.backend {
            WGPUBackendType::Auto => wgpu::BackendBit::PRIMARY,
            WGPUBackendType::Vulkan => wgpu::BackendBit::VULKAN,
            WGPUBackendType::Metal => wgpu::BackendBit::METAL,
            WGPUBackendType::Dx12 => wgpu::BackendBit::DX12,
            WGPUBackendType::Dx11 => wgpu::BackendBit::DX11,
            WGPUBackendType::Gl => wgpu::BackendBit::GL,
            WGPUBackendType::BrowserWGPU => wgpu::BackendBit::BROWSER_WEBGPU,
        };
        let instance = wgpu::Instance::new(backend);
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference : wgpu::PowerPreference::HighPerformance,
            compatible_surface:None
        }).await.expect("Unable to find a GPU! Make sure you have installed required drivers!");
        
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();
       
        let device = Arc::new(device);
        WGPUBackend {
            instance,
            device,
            queue
        }
    }
}

impl IBackend for WGPUBackend {
    fn resource_context(&self) -> &dyn IRenderResourceContext {
        todo!()
    }
}