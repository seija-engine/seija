use image::ImageError;
use seija_core::TypeUuid;
use uuid::Uuid;
use wgpu::{TextureFormat};
use seija_core::bytes::{cast_slice};

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