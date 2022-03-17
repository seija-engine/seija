use std::collections::HashMap;
use std::sync::Arc;
use wgpu::CommandEncoder;

use crate::memory::TypedUniformBuffer;
use crate::resource::RenderResources;
use crate::{UBOInfoSet, UBOInfo};
use super::array_buffer::UBOArrayBuffer;
use super::buffer::UBObject;

#[derive(Clone, Copy,Debug)]
pub struct BufferIndex(usize);

#[derive(Clone, Copy,Debug)]
pub struct BufferArrayIndex(usize,u32);



#[derive(Default)]
pub struct UBOContext {
    pub info:UBOInfoSet,
    pub buffers:BufferContext
}

impl UBOContext {
  
  pub fn init(&mut self) {
       self.buffers.init(&self.info);
  }
  
  pub fn add_camera_buffer(&mut self,name:&str,eid:u32,res:&mut RenderResources) -> Option<BufferIndex> {
    if let Some(info) = self.info.get_info(name) {
      return Some(self.buffers.add_camera_buffer(info, res, eid))
    }
    None
  }

  pub fn add_object_buffer(&mut self,name:&str,eid:u32,res:&mut RenderResources) -> Option<BufferArrayIndex> {
    if let Some(info) = self.info.get_info(name) {
       return Some(self.buffers.add_object_buffer(info, res, eid))
    }
    None
  }
 
  pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
    self.buffers.update(res,cmd);
  }
}

#[derive(Default)]
pub struct BufferContext {
  cameras:Vec<UBObject>,

  object_nameidxs:HashMap<Arc<String>,usize>,
  objects:Vec<UBOArrayBuffer>
}

impl BufferContext {

  pub fn init(&mut self,info_set:&UBOInfoSet) {
    for (_,info) in info_set.per_objects.iter() {
        self.objects.push(UBOArrayBuffer::new(info.props.clone()));
        self.object_nameidxs.insert(info.name.clone(), self.objects.len() - 1);
    }
  }

  pub fn add_camera_buffer(&mut self,info:&UBOInfo,res:&mut RenderResources,eid:u32) -> BufferIndex {
      let object = UBObject::create(info, res);
      self.cameras.push(object);
      BufferIndex(self.cameras.len() - 1)
  }

  pub fn add_object_buffer(&mut self,info:&UBOInfo,res:&mut RenderResources,eid:u32) -> BufferArrayIndex {
      let array_index = self.object_nameidxs.get(&info.name).map(Clone::clone).unwrap_or_default();      
      self.objects[array_index].add_item(eid, res);
      BufferArrayIndex(array_index,eid)
  }

  pub fn get_camera_mut(&mut self,index:&BufferIndex) -> Option<&mut UBObject> {
      self.cameras.get_mut(index.0)
  }

  pub fn get_object_mut(&mut self,index:&BufferArrayIndex) -> Option<&mut TypedUniformBuffer> {
    self.objects.get_mut(index.0)
                .and_then(|buffer| buffer.get_item_buffer_mut(index.1))
  }

  pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
    for camera in self.cameras.iter_mut() {
      camera.update(res,cmd);
    }
    for array in self.objects.iter_mut() {
       array.update(res,cmd);
    }
  }

}