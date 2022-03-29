use std::collections::HashMap;
use seija_core::LogOption;
use wgpu::{CommandEncoder, Device};

use crate::memory::TypedUniformBuffer;
use crate::pipeline::render_bindings::BindGroupLayoutBuilder;
use crate::resource::RenderResources;
use crate::{UBOInfoSet, UBOInfo};
use super::{UBOType, UBObject};
use super::array_buffer::UBOArrayBuffer;
use super::ubo_info::UBOApplyType;

#[derive(Clone, Copy,Debug)]
pub struct BufferIndex(usize);

#[derive(Clone, Copy,Debug)]
pub struct BufferArrayIndex(usize,u32);

pub type UBONameIndex = (UBOType,usize,UBOApplyType);


#[derive(Default)]
pub struct UBOContext {
    pub info:UBOInfoSet,
    pub buffers:BufferContext,
    pub info_layouts:HashMap<String,wgpu::BindGroupLayout>
}

impl UBOContext {
  
  pub fn init(&mut self,device:&wgpu::Device,res:&mut RenderResources) {
      

       for (name,_) in self.info.component_buffers.iter() {
          Self::create_layout(&mut self.info_layouts,name, device);
       }
       for (name,_) in self.info.global_buffers.iter() {
        Self::create_layout(&mut self.info_layouts,name, device);
      }
      self.buffers.init(&self.info,res,&self.info_layouts);
  }

  fn create_layout(layouts:&mut HashMap<String,wgpu::BindGroupLayout>,name:&str,device:&Device) {
     let mut builder = BindGroupLayoutBuilder::new();
     builder.add_uniform(wgpu::ShaderStage::VERTEX);
     let layout = builder.build(device);
     layouts.insert(name.to_string(), layout);
  }

  pub fn add_buffer(&mut self,name:&str,res:&mut RenderResources,eid:Option<u32>)  {
    if let Some(info) = self.info.get_info(name) {
       if let Some(layout) = self.info_layouts.get(name) {
        self.buffers.add_buffer(info,eid ,res,layout);
       }
      
    }
  }
 
  pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
    self.buffers.update(res,cmd);
  }
}

#[derive(Default)]
pub struct BufferContext {
  comp_nameidxs:HashMap<String,UBONameIndex>,
  //Name + EntityId -> Buffer
  components:Vec<UBOArrayBuffer>,

  global_nameidxs:HashMap<String,UBONameIndex>,
  //Name -> Buffer
  globals:Vec<UBObject>
}

impl BufferContext {

  pub fn init(&mut self,info_set:&UBOInfoSet,res:&mut RenderResources,layouts:&HashMap<String,wgpu::BindGroupLayout>) {
    for (_,info) in info_set.component_buffers.iter() {
        self.components.push(UBOArrayBuffer::new(info.props.clone()));
        self.comp_nameidxs.insert(info.name.to_string(), (UBOType::ComponentBuffer,self.components.len() - 1,info.apply));
    }

    for (_,info) in info_set.global_buffers.iter() {
      if let Some(layout) = layouts.get(info.name.as_str()) {
        self.globals.push(UBObject::create(info, res,layout));
        self.global_nameidxs.insert(info.name.to_string(), (UBOType::GlobalBuffer,self.globals.len() - 1,info.apply));
      }
    }
  }

  pub fn add_buffer(&mut self,info:&UBOInfo,m_eid:Option<u32>,res:&mut RenderResources,layout:&wgpu::BindGroupLayout) -> Option<()> {
      match info.typ {
        UBOType::ComponentBuffer => {
          let eid = m_eid.log_err(&format!("ComponentBuffer {} need eid",info.name.as_str()))?;
          let arr_idx = *self.comp_nameidxs.get(info.name.as_str()).log_err(&format!("not found {}",info.name.as_str()))?;
          self.components[arr_idx.1].add_item(eid, res,layout);
          Some(())
        },
        UBOType::GlobalBuffer => {
          Some(())
        }
      }
  }

  pub fn get_name_index(&self,name:&str) -> Option<UBONameIndex> {
      if self.comp_nameidxs.contains_key(name) {
         return self.comp_nameidxs.get(name).map(|v|*v);
      } else {
        return self.global_nameidxs.get(name).map(|v|*v);
      }
      
  }

  pub fn get_buffer_mut(&mut self,name_index:&UBONameIndex,eid:Option<u32>) -> Option<&mut TypedUniformBuffer> {
    match name_index.0 {
        UBOType::ComponentBuffer => {
          let array = &mut self.components[name_index.1];
          array.get_item_buffer_mut(eid.log_err("not found eid in buffer")?)
        },
        UBOType::GlobalBuffer => {
          let ubo = &mut self.globals[name_index.1]; 
          Some(&mut ubo.local) 
        }
    }
  }

  pub fn get_bind_group(&self,name_index:&UBONameIndex,m_eid:Option<u32>) -> Option<&wgpu::BindGroup> {
    match name_index.0 {
      UBOType::ComponentBuffer => {
        let array = &self.components[name_index.1];
        let eid = m_eid.log_err("not found eid in buffer")?;
        let bind_group = array.get_item(eid).map(|v| &v.bind_group);
        if bind_group.is_none() {
          log::error!("{:?}",m_eid);
        }
        bind_group
      },
      UBOType::GlobalBuffer => { 
        let ubo = &self.globals[name_index.1];
        Some(&ubo.bind_group)
      }
  }
  }

  pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
    for arr in self.components.iter_mut() {
      arr.update(res,cmd);
    }
    for global in self.globals.iter_mut() {
      global.update(res,cmd);
    }
  }

}

