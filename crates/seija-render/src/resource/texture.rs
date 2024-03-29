use std::{collections::HashSet};
use bevy_ecs::prelude::World;
use image::{ImageError, ImageFormat};
use seija_asset::{Assets, AssetEvent, Handle};
use uuid::Uuid;
use seija_core::{TypeUuid, IDGenU32, OptionExt};
use bevy_ecs::event::{ManualEventReader, Events};
use once_cell::sync::Lazy;
use wgpu::TextureFormat;
use crate::{resource::{read_image_info, image_info::color_image_info}, RenderContext};
use seija_core::{anyhow::{Result}};
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
        let guess_format = image::guess_format(bytes)?;
        let info = if guess_format == ImageFormat::Hdr {
            let format: TextureFormat = TextureFormat::Rgba32Float;
            let decoder = image::codecs::hdr::HdrDecoder::new(bytes)?;
            let info = decoder.metadata();
            let rgb_data = decoder.read_image_hdr()?;
            let mut rgba_data = Vec::with_capacity(rgb_data.len() * format.describe().block_size as usize);
            for rgb in rgb_data {
                let alpha = 1.0f32;
                rgba_data.extend_from_slice(&rgb.0[0].to_ne_bytes());
                rgba_data.extend_from_slice(&rgb.0[1].to_ne_bytes());
                rgba_data.extend_from_slice(&rgb.0[2].to_ne_bytes());
                rgba_data.extend_from_slice(&alpha.to_ne_bytes());
            }
           ImageInfo {width:info.width,height:info.height,format,data:rgba_data }
        } else {
            let dyn_image = image::load_from_memory(bytes)?;
            read_image_info(dyn_image)
        };
        
        desc.desc.size.width = info.width;
        desc.desc.size.height = info.height;
        desc.desc.size.depth_or_array_layers = 1;
        desc.desc.dimension = wgpu::TextureDimension::D2;
        desc.desc.format = info.format;

        let texture = TextureType::Image(info);
        Ok(Texture {texture,desc })
    }

    pub fn cast_image_data(&self) -> Option<&Vec<u8>> {
        match &self.texture {
            TextureType::Image(image) => Some(&image.data),
            _ => None
        }
    }

    pub fn to_gpu(handle:&Handle<Texture>,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        if ctx.resources.get_render_resource(&handle.id, 0).is_some() {
            return Ok(());
        }
        let textures = world.get_resource::<Assets<Texture>>().get()?;
        if let Some(texture) = textures.get(&handle.id) {
            let desc = texture.desc();
            let texture_id = ctx.resources.create_texture(&desc.desc,&desc.view_desc);
            ctx.resources.set_render_resource(&handle.id, RenderResourceId::TextureView(texture_id), 0);

            let sampler_id = ctx.resources.create_sampler(&desc.sampler_desc);
            ctx.resources.set_render_resource(&handle.id, RenderResourceId::Sampler(sampler_id), 1);
            if let TextureType::Image(_) = texture.texture {
                let command = ctx.command_encoder.as_mut().get()?;
                ctx.resources.fill_texture(texture, &texture_id,command);
            }
        }
        Ok(())
    }

}
#[derive(Debug,Clone)]
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
                view_formats:&[],
                //TODO ?
                usage:wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST
            }, 
            view_desc: wgpu::TextureViewDescriptor::default(), 
            sampler_desc: wgpu::SamplerDescriptor::default() 
        }
    }
}

pub fn color_texture(color:[u8;4],size:usize) -> Texture {
    Texture::create_image(color_image_info(color, size),TextureDescInfo::default())
}

pub fn cube_texture(color:[u8;4],size:usize) -> Texture {
    let mut desc = TextureDescInfo::default();
    desc.desc.size.depth_or_array_layers = 6;
    desc.view_desc.dimension = Some(wgpu::TextureViewDimension::Cube);
    Texture::create_image(color_image_info(color, size),desc)
}

pub fn update_texture_system(world:&mut World,texture_reader:&mut ManualEventReader<AssetEvent<Texture>>,ctx:&mut RenderContext) {
    let texture_events = world.get_resource::<Events<AssetEvent<Texture>>>().unwrap();
    let mut changed_textures:HashSet<Handle<Texture>> = Default::default();
    for event in texture_reader.iter(texture_events) {
        match event {
            AssetEvent::Created { handle } => {
                changed_textures.insert(handle.clone_weak());
            },
            AssetEvent::Modified { .. } => {},
            AssetEvent::Removed { handle,.. } => {
                changed_textures.remove(&handle);
            }
        }
    }
    for texture_handle in changed_textures.iter() {
        if let Err(err) = Texture::to_gpu(&texture_handle, world, ctx) {
            log::error!("upload texture error:{}",err);
        }
    }
    //TODO 这里需要移除释放的Texture的render resource
}