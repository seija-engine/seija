use glam::{Mat4, Vec4, Vec3};
use seija_core::bytes::AsBytes;
use crate::memory::{UniformBufferDef, UniformBuffer};

pub trait IShaderBackend {
    fn from_def(def:&UniformBufferDef) -> Result<Self,String> where Self: Sized;
    fn set_count(&self,buffer:&mut UniformBuffer,count:i32);
}

pub struct Camera3DBackend {
    view_idx:usize,
    proj_idx:usize,
    projview_idx:usize,
    position_idx:usize,
    
}

impl Camera3DBackend {
    pub fn from_def(def:&UniformBufferDef) -> Result<Camera3DBackend,String> {
        let view_idx = def.get_offset("cameraView", 0).ok_or(String::from("cameraView"))?;
        let proj_idx = def.get_offset("cameraProj", 0).ok_or(String::from("cameraProj"))?;
        let projview_idx = def.get_offset("cameraProjView", 0).ok_or(String::from("cameraProjView"))?;
        let position_idx = def.get_offset("cameraPosition", 0).ok_or(String::from("cameraPosition"))?;
        Ok(Camera3DBackend {
            view_idx,
            proj_idx,
            projview_idx,
            position_idx   
        })
    }

    pub fn set_view(&self,buffer:&mut UniformBuffer,mat:&Mat4) {
        buffer.write_bytes_(self.view_idx,  mat.to_cols_array().as_bytes());
    }

    pub fn set_proj(&self,buffer:&mut UniformBuffer,mat:&Mat4) {
        buffer.write_bytes_(self.proj_idx,  mat.to_cols_array().as_bytes());
    }

    pub fn set_projview(&self,buffer:&mut UniformBuffer,mat:&Mat4) {
        buffer.write_bytes_(self.projview_idx,  mat.to_cols_array().as_bytes());
    }

    pub fn set_position(&self,buffer:&mut UniformBuffer,v4:Vec4) {
        buffer.write_bytes(self.position_idx,  v4.to_array());
    }
}

pub struct TransformBackend {
    trans_idx:usize
}

impl TransformBackend {
    pub fn from_def(def:&UniformBufferDef) -> Result<TransformBackend,String> {
        let trans_idx = def.get_offset("transform", 0).ok_or(String::from("transform"))?;
        Ok(TransformBackend {
            trans_idx
        })
    }

    pub fn set_transform(&self,buffer:&mut UniformBuffer,mat:&Mat4) {
        buffer.write_bytes_(self.trans_idx,  mat.to_cols_array().as_bytes());
    }
}
#[allow(dead_code)]
pub struct LightBackend {
    ambile_idx:usize,
    light_count_idx:usize,
    lights_type_idx:usize,
    lights_position_idx:usize,
    lights_item_size:usize,
    lights_direction_idx:usize,
    lights_color_idx:usize,
    lights_intensity_idx:usize,
    lights_ex1_idx:usize,
    lights_ex2_idx:usize,
    lights_ex3_idx:usize,
}
#[allow(dead_code)]
impl LightBackend {
    pub fn from_def(def:&UniformBufferDef) -> Result<LightBackend,String> {
        let ambile_idx = def.get_offset("ambileColor", 0).ok_or("ambileColor".to_string())?;
        let light_count_idx = def.get_offset("lightCount", 0).ok_or("lightCount".to_string())?;

        let lights_position_idx = def.get_array_offset("lights","position", 0).ok_or("lights.position".to_string())?;
        let lights_type_idx = def.get_array_offset("lights","type", 0).ok_or("lights.type".to_string())?;
        
        let lights_direction_idx = def.get_array_offset("lights","direction", 0).ok_or("lights.direction".to_string())?;
        let lights_color_idx = def.get_array_offset("lights","color", 0).ok_or("lights.color".to_string())?;
        let lights_intensity_idx = def.get_array_offset("lights","intensity", 0).ok_or("lights.intensity".to_string())?;
        let lights_ex1_idx = def.get_array_offset("lights","ex1", 0).ok_or("lights.ex1".to_string())?;
        let lights_ex2_idx = def.get_array_offset("lights","ex2", 0).ok_or("lights.ex2".to_string())?;
        let lights_ex3_idx = def.get_array_offset("lights","ex3", 0).ok_or("lights.ex3".to_string())?;
       


        let arr_info = def.get_array_info("lights").ok_or("ok_or".to_string())?;
        Ok(LightBackend {
            ambile_idx,
            light_count_idx,
            lights_type_idx,
            lights_position_idx,
            lights_item_size:arr_info.stride,
            lights_direction_idx,
            lights_color_idx,
            lights_intensity_idx,
            lights_ex1_idx,
            lights_ex2_idx,
            lights_ex3_idx
        })
    }

    pub fn set_ambile_color(&self,buffer:&mut UniformBuffer,color:Vec4) {
        buffer.write_bytes(self.ambile_idx, color.to_array());
    }

    pub fn set_light_count(&self,buffer:&mut UniformBuffer,num:i32) {
        buffer.write_bytes(self.light_count_idx, num);
    }

    

    pub fn set_lights_position(&self,buffer:&mut UniformBuffer,index:usize,pos:Vec3) {
        let offset = self.lights_position_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, pos.to_array());
    }

    pub fn set_lights_type(&self,buffer:&mut UniformBuffer,index:usize,num:i32) {
        let offset = self.lights_type_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, num);
    }

    pub fn set_lights_direction(&self,buffer:&mut UniformBuffer,index:usize,dir:Vec3) {
        let offset = self.lights_direction_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, dir.to_array());
    }

    pub fn set_lights_color(&self,buffer:&mut UniformBuffer,index:usize,color:Vec3) {
        let offset = self.lights_color_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, color.to_array());
    }

    pub fn set_lights_intensity(&self,buffer:&mut UniformBuffer,index:usize,num:f32) {
        let offset = self.lights_intensity_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, num);
    }

    pub fn set_lights_ex1(&self,buffer:&mut UniformBuffer,index:usize,num:f32) {
        let offset = self.lights_ex1_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, num);
    }

    pub fn set_lights_ex2(&self,buffer:&mut UniformBuffer,index:usize,num:f32) {
        let offset = self.lights_ex2_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, num);
    }

    pub fn set_lights_ex3(&self,buffer:&mut UniformBuffer,index:usize,num:f32) {
        let offset = self.lights_ex3_idx + (self.lights_item_size * index * 4);
        buffer.write_bytes(offset, num);
    }
}