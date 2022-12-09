use seija_core::math::{Vec4, Mat4,Vec4Swizzles};

use crate::{types::{Thickness, Rect}, mesh2d::{Mesh2D, Vertex2D}};

use super::rect2d::Rect2D;

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum ImageFilledType {
    HorizontalLeft,
    HorizontalRight,
    VerticalTop,
    VerticalBottom
}

impl From<u32> for ImageFilledType {
    fn from(n: u32) -> ImageFilledType {
        match n {
            0 => ImageFilledType::HorizontalLeft,
            1 => ImageFilledType::HorizontalRight,
            2 => ImageFilledType::VerticalTop,
            _ => ImageFilledType::VerticalBottom,
        }
    }
}

pub enum ImageType {
    Simple,
    Sliced(Thickness),
    Filled(ImageFilledType,f32),
    Tiled,
}

impl Default for ImageType {
    fn default() -> Self {
        ImageType::Simple
    }
}



pub struct ImageGenericInfo {
    pub typ:ImageType,
    pub color:Vec4,
}

impl Default for ImageGenericInfo {
    fn default() -> Self {
        ImageGenericInfo { typ: ImageType::default(), color: Vec4::ONE }
    }
}


impl ImageGenericInfo {
    pub fn build_mesh(&self,mat:&Mat4,rect2d:&Rect2D,uv:Rect<f32>) -> Mesh2D {
        match self.typ {
            ImageType::Simple => { self.build_simple_mesh(mat, rect2d,uv) },
            _ => {
                todo!()
            }
        }
    }

    pub fn build_simple_mesh(&self,mat:&Mat4,rect2d:&Rect2D,uv:Rect<f32>) -> Mesh2D {
        let offset_x = -rect2d.width  * rect2d.anchor[0];
        let offset_y = -rect2d.height * rect2d.anchor[1];
        let indexs = vec![0,1,2,1,3,2];
        
        let points = vec![
              Vertex2D { //left top
                 pos: mat.mul_vec4(Vec4::new(0f32 + offset_x, rect2d.height + offset_y, 0f32,1f32)).xyz(),
                 uv:  [uv.x,uv.y].into()
              },
              Vertex2D { //right top
                pos: mat.mul_vec4(Vec4::new(rect2d.width + offset_x,rect2d.height + offset_y, 0f32,1f32)).xyz(),
                uv:  [uv.x + uv.width,uv.y].into()
             },
             Vertex2D {//left bottom
                pos:  mat.mul_vec4(Vec4::new(0f32 + offset_x,0f32 + offset_y, 0f32,1f32)).xyz(),
                uv:  [uv.x,uv.y + rect2d.height].into()
             },
             Vertex2D {//right bottom
                pos:  mat.mul_vec4(Vec4::new(rect2d.width + offset_x,0f32 + offset_y, 0f32,1f32)).xyz(),
                uv:  [uv.x + uv.width,uv.y + uv.height].into()
             },
        ];
        Mesh2D { 
            color:self.color, 
            points, 
            indexs 
        }
    }
}