use std::path::Path;

use image::ImageError;
use seija_core::bytes::{cast_slice, AsBytes};
use wgpu::TextureFormat;

#[derive(Debug)]
pub struct ImageInfo {
    pub width:u32,
    pub height:u32,
    pub format:TextureFormat,
    pub data:Vec<u8>
}


pub fn load_image_info<P>(path:P) -> Result<ImageInfo,ImageError> where P: AsRef<Path> {
    
    let bytes = std::fs::read(path.as_ref())?;
    
    let dyn_image = image::load_from_memory(&bytes)?;
    let image_info = read_image_info(dyn_image);
    
    Ok(image_info)
}

pub fn read_image_info(dyn_image:image::DynamicImage) -> ImageInfo {
    let data: Vec<u8>;
    let format: TextureFormat;
    let width;
    let height;
    log::error!("color:{:?}",dyn_image.color());
    match dyn_image {
        image::DynamicImage::ImageLuma8(i) => {
            let buffer = i;
            width = buffer.width();
            height = buffer.height();
            format = TextureFormat::R8Unorm;
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
            format = TextureFormat::Rgba8Unorm;
            data = i.into_raw();
        }
        image::DynamicImage::ImageRgba8(i) => {
            width = i.width();
            height = i.height();
            format = TextureFormat::Rgba8UnormSrgb;
            data = i.into_raw();
        }
       
        image::DynamicImage::ImageRgba32F(i) => {},
        image::DynamicImage::ImageRgb32F(i) => {},
       
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
        },
        _ => {}
    }

    ImageInfo {width,height,format,data }
}


pub fn color_image_info(color:[u8;4],size:usize) -> ImageInfo {
    let pixel_len = size * size;
    let mut data:Vec<u8> = Vec::with_capacity(pixel_len * 4);
    for _ in 0..pixel_len {
        data.extend_from_slice(&color);
    }
    
    ImageInfo { width: size as u32, height: size as u32, format: wgpu::TextureFormat::Bgra8Unorm, data }
}