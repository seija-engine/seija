use glam::{Mat4, Vec4};
use seija_core::bytes::AsBytes;
use crate::memory::{UniformBufferDef, UniformBuffer};

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