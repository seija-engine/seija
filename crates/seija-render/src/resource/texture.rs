use std::collections::HashSet;
use bevy_ecs::prelude::World;
use image::ImageError;
use seija_asset::{AssetEvent, Assets, Handle};
use seija_core::{TypeUuid, event::{Events, ManualEventReader}};
use uuid::Uuid;
use wgpu::{TextureFormat};
use seija_core::bytes::{cast_slice};

use crate::RenderContext;

use super::RenderResourceId;

#[derive(Debug, TypeUuid)]
#[uuid = "9fb83fbe-b850-42e0-a58c-53da87aaaa04"]
pub struct Texture {
    pub data: Vec<u8>,
    pub size: wgpu::Extent3d,
    pub format: wgpu::TextureFormat,
    pub dimension: wgpu::TextureDimension,
    pub sampler: wgpu::SamplerDescriptor<'static>,
}

impl Texture {
    pub fn new(size:wgpu::Extent3d,dimension: wgpu::TextureDimension,data: Vec<u8>,format: wgpu::TextureFormat) -> Texture {
        Texture {
            data,
            size,
            format,
            dimension,
            sampler:wgpu::SamplerDescriptor::default()
        }
    }

    pub fn from_bytes(bytes:&[u8]) -> Result<Texture,ImageError> {
        let images = image::load_from_memory(bytes)?;
        Ok(images.into())
    }

    pub fn desc(&self,usage:wgpu::TextureUsage) -> wgpu::TextureDescriptor {
        wgpu::TextureDescriptor {
            label:None,
            size:self.size,
            mip_level_count:1,
            sample_count:1,
            dimension:self.dimension,
            format:self.format,
            usage
        }
    }

    pub fn view_desc(&self) -> wgpu::TextureViewDescriptor {
        wgpu::TextureViewDescriptor {
            label:None,
            format:Some(self.format),
            dimension:Some(wgpu::TextureViewDimension::D2),

            ..Default::default()
        }
    }
}


impl From<image::DynamicImage> for Texture {
    fn from(dyn_img: image::DynamicImage) -> Texture {
        let data: Vec<u8>;
        let format: TextureFormat;
        let width;
        let height;

        match dyn_img {
            image::DynamicImage::ImageLuma8(i) => {
                let buffer = image::DynamicImage::ImageLuma8(i).into_rgba8();
                width = buffer.width();
                height = buffer.height();
                format = TextureFormat::Rgba8UnormSrgb;
                data = buffer.into_raw();
            },
            image::DynamicImage::ImageLumaA8(i) => {
                let i = image::DynamicImage::ImageLumaA8(i).into_rgba8();
                width = i.width();
                height = i.height();
                format = TextureFormat::Rgba8UnormSrgb;
                data = i.into_raw();
            }
            ,image::DynamicImage::ImageRgb8(i) => {
                let i = image::DynamicImage::ImageRgb8(i).into_rgba8();
                width = i.width();
                height = i.height();
                format = TextureFormat::Rgba8UnormSrgb;
                data = i.into_raw();
            }
            image::DynamicImage::ImageRgba8(i) => {
                width = i.width();
                height = i.height();
                format = TextureFormat::Rgba8UnormSrgb;
                data = i.into_raw();
            }
            image::DynamicImage::ImageBgr8(i) => {
                let i = image::DynamicImage::ImageBgr8(i).into_bgra8();

                width = i.width();
                height = i.height();
                format = TextureFormat::Bgra8UnormSrgb;
                data = i.into_raw();
            }
            image::DynamicImage::ImageBgra8(i) => {
                width = i.width();
                height = i.height();
                format = TextureFormat::Bgra8UnormSrgb;
                data = i.into_raw();
            }
            image::DynamicImage::ImageLuma16(i) => {
                width = i.width();
                height = i.height();
                format = TextureFormat::R16Uint;
                let raw_data = i.into_raw();
                data = cast_slice(&raw_data).to_owned();
            }
            image::DynamicImage::ImageLumaA16(i) => {
                width = i.width();
                height = i.height();
                format = TextureFormat::Rg16Uint;
                let raw_data = i.into_raw();
                data = cast_slice(&raw_data).to_owned();
            },
            image::DynamicImage::ImageRgb16(image) => {
                width = image.width();
                height = image.height();
                format = TextureFormat::Rgba16Uint;
                let mut local_data = Vec::with_capacity(width as usize * height as usize * format.describe().block_size as usize);
                for pixel in image.into_raw().chunks_exact(3) {
                    let r = pixel[0];
                    let g = pixel[1];
                    let b = pixel[2];
                    let a = u16::MAX;
                    local_data.extend_from_slice(&r.to_ne_bytes());
                    local_data.extend_from_slice(&g.to_ne_bytes());
                    local_data.extend_from_slice(&b.to_ne_bytes());
                    local_data.extend_from_slice(&a.to_ne_bytes());
                }

                data = local_data;
            },
            image::DynamicImage::ImageRgba16(i) => {
                width = i.width();
                height = i.height();
                format = TextureFormat::Rgba16Uint;
                let raw_data = i.into_raw();
                data = cast_slice(&raw_data).to_owned();
            }
        }

        Texture::new(wgpu::Extent3d {width,height,depth_or_array_layers:1},wgpu::TextureDimension::D2,data,format)
    }
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
            let texture_id = ctx.resources.create_texture(&texture.desc(wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST),&texture.view_desc());
            ctx.resources.set_render_resource(texture_handle.clone_weak_untyped(), RenderResourceId::Texture(texture_id), 0);

            let sampler_id = ctx.resources.create_sampler(&texture.sampler);
            ctx.resources.set_render_resource(texture_handle.clone_weak_untyped(), RenderResourceId::Sampler(sampler_id), 1);

          
           ctx.resources.fill_texture(texture, &texture_id,command);
        }
    }
}