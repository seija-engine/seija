use seija_core::{TypeUuid};
use uuid::Uuid;
use wgpu::Device;

#[derive(Debug,TypeUuid)]
#[uuid = "9fb83fbe-b850-42e0-a58c-53da87aaaa04"]
pub struct Texture {
    pub data: Vec<u8>,
    size:wgpu::Extent3d
}

impl Texture {
    pub fn new(device:&Device) {
       //device.create_texture(wgpu::TextureDescriptor {})
    }
}