use seija_core::math::{Vec4, Mat4,Vec4Swizzles, Vec2};
use crate::common::types::Thickness;
use seija_core::Rect;
use crate::common::{Rect2D,Mesh2D,Vertex2D};

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


#[repr(C)]
pub struct ImageGenericInfo {
    pub color:Vec4,
    pub typ:ImageType,
}

impl Default for ImageGenericInfo {
    fn default() -> Self {
        ImageGenericInfo { typ: ImageType::default(), color: Vec4::ONE }
    }
}


impl ImageGenericInfo {
    pub fn build_mesh(&self,mat:&Mat4,rect2d:&Rect2D,uv:&Rect<f32>,raw_size:&Rect<u32>,z_order:f32) -> Mesh2D {
        match &self.typ {
            ImageType::Simple => { self.build_simple_mesh(mat,rect2d,uv,z_order) },
            ImageType::Sliced(thinkness) => { self.build_slice_mesh(thinkness,mat,rect2d,uv,raw_size,z_order) },
            _ => {
                todo!()
            }
        }
    }

    pub fn build_simple_mesh(&self,mat:&Mat4,rect2d:&Rect2D,uv:&Rect<f32>,z_order:f32) -> Mesh2D {
        let offset_x = -rect2d.width  * rect2d.anchor[0];
        let offset_y = -rect2d.height * rect2d.anchor[1];
        let indexs = vec![2,1,0,2,3,1];
        
        let points = vec![
              Vertex2D { //left top
                 pos: mat.mul_vec4(Vec4::new(0f32 + offset_x, rect2d.height + offset_y, z_order,1f32)).xyz(),
                 uv:  [uv.x,uv.y].into(),
              },
              Vertex2D { //right top
                pos: mat.mul_vec4(Vec4::new(rect2d.width + offset_x,rect2d.height + offset_y,z_order,1f32)).xyz(),
                uv:  [uv.x + uv.width,uv.y].into(),
             },
             Vertex2D {//left bottom
                pos:  mat.mul_vec4(Vec4::new(0f32 + offset_x,0f32 + offset_y,z_order,1f32)).xyz(),
                uv:  [uv.x,uv.y + uv.height].into(),
             },
             Vertex2D {//right bottom
                pos:  mat.mul_vec4(Vec4::new(rect2d.width + offset_x,0f32 + offset_y,z_order,1f32)).xyz(),
                uv:  [uv.x + uv.width,uv.y + uv.height].into(),
             },
        ];

        Mesh2D { 
            color:self.color, 
            points, 
            indexs 
        }
    }

    pub fn build_slice_mesh(&self,thickness:&Thickness,mat:&Mat4,rect2d:&Rect2D,uv:&Rect<f32>,raw_size:&Rect<u32>,z_order:f32) -> Mesh2D {
      
        let offset_x = rect2d.width  * rect2d.anchor[0];
        let offset_y = rect2d.height * rect2d.anchor[1];
        //left -> right
        let (x0,x1,x2,x3) = (-offset_x, -offset_x + thickness.left,-offset_x + rect2d.width -thickness.right,-offset_x + rect2d.width);
        //top -> bottom
        let (y0,y1,y2,y3) = (-offset_y + rect2d.height,-offset_y + rect2d.height - thickness.top ,-offset_y + thickness.bottom, -offset_y);

        let  left_uv_width  = uv.width * (thickness.left / raw_size.width as f32);
        let  right_uv_width = uv.width * (thickness.right / raw_size.width as f32);
        let  top_uv_height = uv.height * (thickness.top / raw_size.height as f32);
        let  bottom_uv_height = uv.height * (thickness.bottom / raw_size.height as f32);

        let (uv_x0,uv_x1,uv_x2,uv_x3) = (uv.x,uv.x + left_uv_width,uv.x + uv.width  - right_uv_width,uv.x + uv.width);
        let (uv_y0,uv_y1,uv_y2,uv_y3) = (uv.y,uv.y + top_uv_height,uv.y + uv.height - bottom_uv_height,uv.y + uv.height);

        let mut ia = 0;
        let mut meshes:Vec<Vertex2D> = vec![];
        let mut indexs:Vec<u32> = vec![];
        //center
        ImageGenericInfo::fill_quad_mesh(&mut meshes,mat,x1,x2,y1,y2,uv_x1,uv_x2,uv_y1,uv_y2, z_order);
        ImageGenericInfo::fill_quad_index(&mut indexs, ia);
        //left top
        if thickness.left > 0f32 && thickness.top > 0f32 {
            ImageGenericInfo::fill_quad_mesh(&mut meshes,mat,x0,x1,y0,y1,uv_x0,uv_x1,uv_y0,uv_y1, z_order);
            ia += 4;
            ImageGenericInfo::fill_quad_index(&mut indexs, ia);
        }
        //top
        if thickness.top > 0f32 {
            ImageGenericInfo::fill_quad_mesh(&mut meshes,mat,x1,x2,y0,y1,uv_x1,uv_x2,uv_y0,uv_y1,z_order);
            ia += 4;
            ImageGenericInfo::fill_quad_index(&mut indexs, ia);
        }
        //right top
        if thickness.right > 0f32 && thickness.top > 0f32 {
            ImageGenericInfo::fill_quad_mesh(&mut meshes,mat,x2,x3,y0,y1,uv_x2,uv_x3,uv_y0,uv_y1,z_order);
            ia += 4;
            ImageGenericInfo::fill_quad_index(&mut indexs,ia);
        }
        //left
        if thickness.left > 0f32 {
            ImageGenericInfo::fill_quad_mesh(&mut meshes,mat,x0,x1,y1,y2,uv_x0,uv_x1,uv_y1,uv_y2,z_order);
            ia += 4;
            ImageGenericInfo::fill_quad_index(&mut indexs,ia);
        }
        //right
        if thickness.right > 0f32 {
            ImageGenericInfo::fill_quad_mesh(&mut meshes,mat,x2,x3,y1,y2,uv_x2,uv_x3,uv_y1,uv_y2,z_order);
            ia += 4;
            ImageGenericInfo::fill_quad_index(&mut indexs,ia);
        }
        //bottom left
        if thickness.bottom > 0f32 && thickness.left > 0f32 {
            ImageGenericInfo::fill_quad_mesh(&mut meshes,mat,x0,x1,y2,y3,uv_x0,uv_x1,uv_y2,uv_y3,z_order);
            ia += 4;
            ImageGenericInfo::fill_quad_index(&mut indexs,ia);
        }
        //bottom
        if thickness.bottom > 0f32 {
            ImageGenericInfo::fill_quad_mesh(&mut meshes,mat,x1,x2,y2,y3,uv_x1,uv_x2,uv_y2,uv_y3,z_order);
            ia += 4;
            ImageGenericInfo::fill_quad_index(&mut indexs,ia);
        }
        //bottom right
        if thickness.right > 0f32 && thickness.bottom > 0f32 {
            ImageGenericInfo::fill_quad_mesh(&mut meshes,mat,x2,x3,y2,y3,uv_x2,uv_x3,uv_y2,uv_y3,z_order);
            ia += 4;
            ImageGenericInfo::fill_quad_index(&mut indexs,ia);
        }

        Mesh2D {
          color:self.color,
          points:meshes,
          indexs  
        }
    }

    pub fn fill_quad_mesh(verts:&mut Vec<Vertex2D>,mat:&Mat4,l:f32,r:f32,t:f32,b:f32,uv_l:f32,uv_r:f32,uv_t:f32,uv_b:f32,z:f32) {
        //left top
        verts.push(Vertex2D { pos: mat.mul_vec4(Vec4::new(l, t, z, 1f32)).xyz(), uv: Vec2::new(uv_l, uv_t)});
        //right top
        verts.push(Vertex2D { pos: mat.mul_vec4(Vec4::new(r, t, z, 1f32)).xyz(), uv: Vec2::new(uv_r, uv_t) });
        //left bottom
        verts.push(Vertex2D { pos: mat.mul_vec4(Vec4::new(l, b, z, 1f32)).xyz(), uv: Vec2::new(uv_l, uv_b)});
        //right bottom
        verts.push(Vertex2D { pos: mat.mul_vec4(Vec4::new(r, b, z, 1f32)).xyz(), uv: Vec2::new(uv_r, uv_b) });
    }

    fn fill_quad_index(indexs:&mut Vec<u32>,start:u32) {
        indexs.extend_from_slice(&[ start + 2,start + 1,start + 0,start + 2,start + 3,start + 1]);
    }
}
