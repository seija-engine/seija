use std::{ path::Path};

use image::ImageError;

use super::{ImageInfo, Texture, load_image_info, TextureDescInfo};

#[derive(Debug,Default)]
pub struct CubeMapBuilder {
    images:Vec<Option<ImageInfo>>,
}

impl CubeMapBuilder {
    pub fn new() -> CubeMapBuilder {
        let mut images:Vec<Option<ImageInfo>> = Vec::with_capacity(6);
        for _ in 0..6 {
            images.push(None);
        }
        CubeMapBuilder { images }
    }

    
    pub fn with_left_path<P>(mut self,path:P) -> Result<CubeMapBuilder,ImageError>  where P :AsRef<Path> {
        self.images[0] = Some(load_image_info(path)?);
        Ok(self)
    }
    
    pub fn with_right_path<P>(mut self,path:P) -> Result<CubeMapBuilder,ImageError> where P:AsRef<Path> {    
        self.images[1] = Some(load_image_info(path)?);
        Ok(self)
    }

    pub fn with_top_path<P>(mut self,path:P) -> Result<CubeMapBuilder,ImageError> where P:AsRef<Path> {    
        self.images[2] = Some(load_image_info(path)?);
        Ok(self)
    }

    pub fn with_bottom_path<P>(mut self,path:P) -> Result<CubeMapBuilder,ImageError> where P:AsRef<Path> {    
        self.images[3] = Some(load_image_info(path)?);
        Ok(self)
    }

    pub fn with_back_path<P>(mut self,path:P) -> Result<CubeMapBuilder,ImageError> where P:AsRef<Path> {    
        self.images[4] = Some(load_image_info(path)?);
        Ok(self)
    }

    pub fn with_front_path<P>(mut self,path:P) -> Result<CubeMapBuilder,ImageError> where P:AsRef<Path> {    
        self.images[5] = Some(load_image_info(path)?);
        Ok(self)
    }

    pub fn build(self) -> Option<Texture> {
        let fst = self.images[0].as_ref()?;
        let byte_length = fst.data.len();
        let mut all_bytes:Vec<u8> = vec![0;fst.data.len() * 6];
        for index in 0..6usize {
            let start = index * byte_length;
            let end = start + byte_length;
            let data_ref = &self.images[index].as_ref()?.data;
            all_bytes[start..end].clone_from_slice(data_ref);
        }
        
        let info = ImageInfo {
            width:fst.width,
            height:fst.height,
            format:fst.format,
            data:all_bytes
        };
        let mut desc = TextureDescInfo::default();
        desc.desc.size.depth_or_array_layers = 6;
        let texture = Texture::create_image(info, desc);
        Some(texture)
    }
}