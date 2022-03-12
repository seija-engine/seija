use std::collections::HashMap;
use std::sync::Arc;
use wgpu::CommandEncoder;

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
  pub fn add(&mut self,name:&str,eid:Option<u32>,res:&mut RenderResources) -> Option<UBOKey> {
      if let Some(info) = self.info.get_info(name) {
        return Some(self.buffers.add(info, res, eid))
      }
      None
  }
  
  pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
    self.buffers.update(res,cmd);
  }
}


#[derive(Clone, Copy,Debug)]
pub struct UBOKey(UBOType,usize);

#[derive(Default)]
pub struct BufferContext {
  keys:HashMap<(Arc<String>,u32),UBOKey>,
  cameras:Vec<UBObject>,
  frames:Vec<UBObject>,
  objects:Vec<UBObject>
}

impl BufferContext {
  pub fn add(&mut self,info:&UBOInfo,res:&mut RenderResources,eid:Option<u32>) -> UBOKey {
    let object = UBObject::create(info, res);
    let key = match info.typ {
      UBOType::PerCamera => {
        self.cameras.push(object);
        UBOKey(UBOType::PerCamera,self.cameras.len() - 1)
      },
      UBOType::PerFrame => {
        self.frames.push(object);
        UBOKey(UBOType::PerFrame,self.frames.len() - 1)
      },
      UBOType::PerObject => {
        self.objects.push(object);
        UBOKey(UBOType::PerObject,self.objects.len() - 1)
      },
    };
    self.keys.insert((info.name.clone(),eid.unwrap_or(0)), key);
   
    log::info!("add ubo buffer {}",info.name.as_str());
    key
  }

  pub fn get_ubo_mut(&mut self,key:&UBOKey) -> Option<&mut UBObject> {
    match key.0 {
      UBOType::PerCamera => self.cameras.get_mut(key.1),
      UBOType::PerFrame  =>  self.frames.get_mut(key.1),
      UBOType::PerObject => self.objects.get_mut(key.1),
    }
  }


  pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
    for camera in self.cameras.iter_mut() {
      camera.update(res,cmd);
    }
    for object in self.objects.iter_mut() {
      object.update(res,cmd);
    }
    for frame in self.frames.iter_mut() {
      frame.update(res,cmd);
    }
  }
}