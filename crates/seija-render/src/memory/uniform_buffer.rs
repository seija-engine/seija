use std::{alloc, convert::TryFrom, sync::Arc};
use seija_core::bytes::AsBytes;
use glam::{Mat3, Mat4};
use serde_json::Value;

use super::uniform_buffer_def::{UniformBufferDef, UniformType};

pub struct UniformBuffer {
    bytes:Vec<u8>
}

impl UniformBuffer {
    pub fn new(size:usize) -> UniformBuffer {
        let bytes = vec![0u8;size];
        UniformBuffer {
            bytes
        }
    }

    pub fn write_bytes<T:AsBytes>(&mut self,offset:usize,v:T) {
        let bytes = v.as_bytes();
       
        self.bytes[offset..(offset + bytes.len())].copy_from_slice(bytes);
    }

    pub fn write_bytes_(&mut self,offset:usize,bytes:&[u8]) {
        self.bytes[offset..(offset + bytes.len())].copy_from_slice(bytes);
    }
    

    pub fn read_bytes<T>(&self,offset:usize,size:usize) -> T {
        let slice = &self.bytes[offset..(offset + size)];
        unsafe { slice.as_ptr().cast::<T>().read_unaligned() }
    }
}

pub struct TypedUniformBuffer {
    def:Arc<UniformBufferDef>,
    buffer:UniformBuffer
}

impl TypedUniformBuffer {
    pub fn from_def(def:Arc<UniformBufferDef>) -> TypedUniformBuffer  {
       let mut ret = TypedUniformBuffer {
           buffer:UniformBuffer::new(def.size()),
           def,
       };
       ret.set_default();
       ret
    }

    pub fn set_default(&mut self) {
        for info in self.def.infos.iter() {
            match &info.typ {
                UniformType::INT(arr) => {
                    for idx in 0..info.size {
                        let v = arr.get(idx).map(|v|v.clone()).unwrap_or(0i32);
                        self.buffer.write_bytes(info.get_buffer_offset(idx), v);
                    }
                },
                UniformType::FLOAT(arr) => {
                    for idx in 0..info.size {
                        let v = arr.get(idx).map(|v|v.clone()).unwrap_or(0f32);
                        self.buffer.write_bytes(info.get_buffer_offset(idx), v);
                    }
                },
                UniformType::UINT(arr) => {
                    for idx in 0..info.size {
                        let v = arr.get(idx).map(|v|v.clone()).unwrap_or(0u32);
                        self.buffer.write_bytes(info.get_buffer_offset(idx), v);
                    }
                },
                UniformType::BOOL(arr) => {
                    for idx in 0..info.size {
                        let v = arr.get(idx).map(|v|v.clone()).unwrap_or(false);
                        let u:u32 = if v {1u32 } else {0u32 };
                        self.buffer.write_bytes(info.get_buffer_offset(idx), u);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn set_f32(&mut self,name:&str,v:f32,idx:usize) {
        if let Some(offset) = self.def.get_offset(name, idx) {
            self.buffer.write_bytes(offset, v);
        }
    }

    pub fn get_f32(&self,name:&str,idx:usize) -> f32 {
        if let Some(offset) = self.def.get_offset(name, idx) {
            return self.buffer.read_bytes(offset,4);
        }
        0f32
    }

    pub fn set_i32(&mut self,name:&str,v:i32,idx:usize) {
        if let Some(offset) = self.def.get_offset(name, idx) {
            self.buffer.write_bytes(offset, v);
        }
    }

    pub fn get_i32(&self,name:&str,idx:usize) -> i32 {
        if let Some(offset) = self.def.get_offset(name, idx) {
            return self.buffer.read_bytes(offset,4);
        }
        0i32
    }

    pub fn set_u32(&mut self,name:&str,v:u32,idx:usize) {
        if let Some(offset) = self.def.get_offset(name, idx) {
            self.buffer.write_bytes(offset, v);
        }
    }

    pub fn get_u32(&self,name:&str,idx:usize) -> u32 {
        if let Some(offset) = self.def.get_offset(name, idx) {
            return self.buffer.read_bytes(offset,4);
        }
        0u32
    }

    pub fn set_bool(&mut self,name:&str,v:bool,idx:usize) {
        if let Some(offset) = self.def.get_offset(name, idx) {
            let v = if v { 1u32 } else { 0u32 };
            self.buffer.write_bytes(offset, v);
        }
    }

    pub fn get_bool(&self,name:&str,idx:usize) -> bool {
        if let Some(offset) = self.def.get_offset(name, idx) {
            let v:u32 = self.buffer.read_bytes(offset,4);
           return v == 1
        }
        false
    }

    pub fn set_mat4(&mut self,name:&str,mat:&Mat4,idx:usize) {
        if let Some(offset) = self.def.get_offset(name, idx) {
            self.buffer.write_bytes_(offset, mat.to_cols_array().as_bytes());
        }
    }

    pub fn get_mat4(&self,name:&str,idx:usize) -> Mat4 {
        if let Some(offset) = self.def.get_offset(name, idx) {
            let bytes:[f32;16] = self.buffer.read_bytes(offset, 64);
            return Mat4::from_cols_array(&bytes)
        }
        Mat4::IDENTITY
    }

    pub fn set_mat3(&mut self,name:&str,mat:&Mat3,idx:usize) {
        if let Some(offset) = self.def.get_offset(name, idx) {
            self.buffer.write_bytes_(offset, mat.to_cols_array().as_bytes());
         }
    }

    pub fn get_mat3(&self,name:&str,idx:usize) -> Mat3 {
        if let Some(offset) = self.def.get_offset(name, idx) {
            let bytes:[f32;9] = self.buffer.read_bytes(offset, 36);
            return Mat3::from_cols_array(&bytes)
        }
        Mat3::IDENTITY
    }

}

#[test]
fn test() {
    let json_string = r#"
      [
          {":name": "radius", ":type": "float", ":default": 6 },
          {":name": "intNumber", ":type": "int[2]",":default":[123,-456] },
          {":name": "uValue", ":type": "uint", ":default": 777 },
          {":name": "bValue", ":type": "bool", ":default": true },

          {":name": "matValue", ":type": "mat4" },
          {":name": "mat3Value", ":type": "mat3" }
      ]
    "#;
    let v:Value = serde_json::from_str(&json_string).unwrap();
    let udef = Arc::new(UniformBufferDef::try_from(&v).unwrap());
    
    let mut typed_buffer = TypedUniformBuffer::from_def(udef.clone());
    
    //typed_buffer.set_f32("radius", 3.1415926f32,0);
    let v0 = typed_buffer.get_f32("radius",0);
    println!("radius:{}",v0);

    //typed_buffer.set_i32("intNumber", 667,1);
    //typed_buffer.set_i32("intNumber", -123,0);
    let v1_1 = typed_buffer.get_i32("intNumber",1);
    let v1_0 = typed_buffer.get_i32("intNumber",0);
    println!("intNumber:{} {}",v1_0,v1_1);
    
    //typed_buffer.set_u32("uValue", 8883,0);
    let v2 = typed_buffer.get_u32("uValue",0);
    println!("uValue:{}",v2);

    //typed_buffer.set_bool("bValue", true,0);
    let v3 = typed_buffer.get_bool("bValue",0);
    println!("bValue:{}",v3);

    //typed_buffer.set_mat4("matValue", &Mat4::IDENTITY,0);
    let v4 = typed_buffer.get_mat4("matValue",0);
    println!("matValue:{:?}",v4);

    //typed_buffer.set_mat3("mat3Value", &Mat3::IDENTITY,0);
    let v5 = typed_buffer.get_mat3("mat3Value",0);
    println!("mat3Value:{:?}",v5);

    println!("bytes:{:?}",&typed_buffer.buffer.bytes);
}
