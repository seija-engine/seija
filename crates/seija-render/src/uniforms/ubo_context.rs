use std::collections::HashMap;

use crate::resource::RenderResources;
use crate::{UBOInfoSet, UBOInfo};
use super::buffer::UBObject;
use super::ubo_info::{UBOType};
#[derive(Default)]
pub struct UBOContext {
    pub info:UBOInfoSet,
    pub buffers:BufferContext
}

impl UBOContext {
   
}

pub struct UBOKey(UBOType,usize);

#[derive(Default)]
pub struct BufferContext {
  keys:HashMap<(String,u32),UBOKey>,
  camera:Vec<UBObject>
}

impl BufferContext {
  pub fn add(&mut self,info:&UBOInfo,res:&mut RenderResources) -> UBOKey {
    let object = UBObject::create(info, res);
    
    match info.typ {
      UBOType::PerCamera => {
        self.camera.push(object);
        UBOKey(UBOType::PerCamera,self.camera.len() - 1)
      },
      UBOType::PerFrame => {
        todo!()
      },
      UBOType::PerObject => {
        todo!()
      },
    }
  }
}


