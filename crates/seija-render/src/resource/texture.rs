use std::collections::HashSet;

use bevy_ecs::prelude::World;
use image::ImageError;
use seija_asset::{Assets, AssetEvent, Handle};
use serde::__private::de;
use uuid::Uuid;
use seija_core::{TypeUuid, IDGenU32, event::{ManualEventReader, Events}};
use once_cell::sync::Lazy;
use crate::{resource::{read_image_info, image_info::color_image_info}, RenderContext};

use super::{ImageInfo, RenderResourceId};

static IDGEN_TEXTURE:Lazy<IDGenU32> = Lazy::new(|| { IDGenU32::new() });

#[derive(TypeUuid,Debug)]
#[uuid = "9fb83fbe-b850-42e0-a58c-53da87aaaa05"]
pub struct Texture {
    pub texture:TextureType,
    desc:TextureDescInfo
}

#[derive(Debug)]
pub enum TextureType {
    Image(ImageInfo),
    RenderTexture(u32)
}

impl Texture {
    pub fn create_by_desc(desc:TextureDescInfo) -> Texture {
        let id = IDGEN_TEXTURE.next();
        let rt = TextureType::RenderTexture(id);
        Texture { texture:rt, desc }
    }
    
    pub fn desc(&self) -> &TextureDescInfo { &self.desc }

    pub fn create_image(image:ImageInfo,mut desc:TextureDescInfo) -> Texture {
        desc.desc.size.width = image.width;
        desc.desc.size.height = image.height;
        desc.desc.format = image.format;
        let texture = TextureType::Image(image);
        Texture {texture,desc }
    }

    pub fn from_image_bytes(bytes:&[u8],mut desc:TextureDescInfo) -> Result<Texture,ImageError> {
        let dyn_image = image::load_from_memory(bytes)?;
        let info = read_image_info(dyn_image);
        desc.desc.size.width = info.width;
        desc.desc.size.height = info.height;
        desc.desc.size.depth_or_array_layers = 1;
        desc.desc.dimension = wgpu::TextureDimension::D2;
        desc.desc.format = info.format;

        let texture = TextureType::Image(info);
        Ok(Texture {texture,desc })
    }
}
#[derive(Debug)]
pub struct TextureDescInfo {
   pub desc:wgpu::TextureDescriptor<'static>,
   pub view_desc:wgpu::TextureViewDescriptor<'static>,
   pub sampler_desc:wgpu::SamplerDescriptor<'static>
}

impl Default for TextureDescInfo {
    fn default() -> Self {
        TextureDescInfo { 
            desc: wgpu::TextureDescriptor {
                label:None,
                size:wgpu::Extent3d::default(),
                mip_level_count:1,
                sample_count:1,
                dimension:wgpu::TextureDimension::D2,
                format:wgpu::TextureFormat::Rgba32Float,
                usage:wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST
            }, 
            view_desc: wgpu::TextureViewDescriptor::default(), 
            sampler_desc: wgpu::SamplerDescriptor::default() 
        }
    }
}

pub fn color_texture(color:[u8;4],size:usize) -> Texture {
    Texture::create_image(color_image_info(color, size),TextureDescInfo::default())
}


pub fn update_texture_system(world:&mut World,texture_reader:&mut ManualEventReader<AssetEvent<Texture>>,ctx:&mut RenderContext) {
    let command = ctx.command_encoder.as_mut().unwrap();
    let texture_events = world.get_resource::<Events<AssetEvent<Texture>>>().unwrap();
    let mut changed_textures:HashSet<Handle<Texture>> = Default::default();
    for event in texture_reader.iter(texture_events) {
        match event {
            AssetEvent::Created { handle } => {
                changed_textures.insert(handle.clone_weak());
            },
            AssetEvent::Modified { .. } => {},
            AssetEvent::Removed { handle } => {
                changed_textures.remove(&handle);
            }
        }
    }

    let textures = world.get_resource::<Assets<Texture>>().unwrap();
    for texture_handle in changed_textures.iter() {
        if let Some(texture) = textures.get(&texture_handle.id) {
            let desc = texture.desc();
            let texture_id = ctx.resources.create_texture(&desc.desc,&desc.view_desc);
            ctx.resources.set_render_resource(&texture_handle.id, RenderResourceId::TextureView(texture_id), 0);

            let sampler_id = ctx.resources.create_sampler(&desc.sampler_desc);
            ctx.resources.set_render_resource(&texture_handle.id, RenderResourceId::Sampler(sampler_id), 1);
            if let TextureType::Image(_) = texture.texture {
                ctx.resources.fill_texture(texture, &texture_id,command);
            }
        }
    }
}