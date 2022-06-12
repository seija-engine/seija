use std::collections::HashMap;
use bevy_ecs::schedule::GraphNode;
use seija_core::LogOption;
use wgpu::{CommandEncoder, Device};

use crate::memory::TypedUniformBuffer;
use crate::pipeline::render_bindings::BindGroupLayoutBuilder;
use crate::resource::RenderResources;
use crate::{UniformInfoSet, UniformInfo};
use super::{UBOType, UBObject};
use super::array_buffer::UBOArrayBuffer;
use super::uniform_info::UBOApplyType;

#[derive(Clone, Copy,Debug)]
pub struct BufferIndex(usize);

#[derive(Clone, Copy,Debug)]
pub struct BufferArrayIndex(usize,u32);

pub type UBONameIndex = (UBOType,usize,UBOApplyType);


#[derive(Default)]
pub struct UniformContext {
    pub info:UniformInfoSet,
    pub info_layouts:HashMap<String,wgpu::BindGroupLayout>,
    pub buffers:BufferContext,
}

impl UniformContext {
  
  pub fn init(&mut self,device:&wgpu::Device,res:&mut RenderResources) {
      

       for (name,info) in self.info.component_buffers.iter() {
          Self::create_layout(&mut self.info_layouts,name, device,info);
       }
       for (name,info) in self.info.global_buffers.iter() {
        Self::create_layout(&mut self.info_layouts,name, device,info);
      }
      self.buffers.init(&self.info,res,&self.info_layouts);
  }

  fn create_layout(layouts:&mut HashMap<String,wgpu::BindGroupLayout>,name:&str,device:&Device,info:&UniformInfo) {
     let mut builder = BindGroupLayoutBuilder::new();
     builder.add_uniform(info.shader_stage);
     
     for texture_desc in info.textures.iter() {
        let desc = texture_desc.desc.0.format.describe();
        
        builder.add_texture(false,Some(desc.sample_type));
        let filtering = if let wgpu::TextureSampleType::Float { filterable } = desc.sample_type {
          filterable
        } else {
          false
        };
        builder.add_sampler(filtering);
     }

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

  pub fn init(&mut self,info_set:&UniformInfoSet,res:&mut RenderResources,layouts:&HashMap<String,wgpu::BindGroupLayout>) {
    for (_,info) in info_set.component_buffers.iter() {
        self.components.push(UBOArrayBuffer::new(info.props.clone()));
        self.comp_nameidxs.insert(info.name.to_string(), (UBOType::Component,self.components.len() - 1,info.apply));
    }

    for (_,info) in info_set.global_buffers.iter() {
      if let Some(layout) = layouts.get(info.name.as_str()) {
        self.globals.push(UBObject::create(info, res,layout));
        self.global_nameidxs.insert(info.name.to_string(), (UBOType::Global,self.globals.len() - 1,info.apply));
      }
    }
  }

  pub fn add_buffer(&mut self,info:&UniformInfo,m_eid:Option<u32>,res:&mut RenderResources,layout:&wgpu::BindGroupLayout) -> Option<()> {
      match info.typ {
        UBOType::Component => {
          let eid = m_eid.log_err(&format!("ComponentBuffer {} need eid",info.name.as_str()))?;
          let arr_idx = *self.comp_nameidxs.get(info.name.as_str()).log_err(&format!("not found {}",info.name.as_str()))?;
          self.components[arr_idx.1].add_item(eid, res,layout);
          Some(())
        },
        UBOType::Global => {
          Some(())
        }
      }
  }

  pub fn remove_buffer_item(&mut self,name:&str,eid:u32) {
    if let Some(index) = self.comp_nameidxs.get(name).map(|v| v.1) {
       self.remove_buffer_item_byindex(index, eid);
    }
  }

  pub fn remove_buffer_item_byindex(&mut self,index:usize,eid:u32) {
    if self.components.len() > index {
      let arr_buffer = &mut self.components[index];
      arr_buffer.remove_item(eid);
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
        UBOType::Component => {
          let array = &mut self.components[name_index.1];
          array.get_item_buffer_mut(eid.log_err("not found eid in buffer")?)
        },
        UBOType::Global => {
          let ubo = &mut self.globals[name_index.1]; 
          Some(&mut ubo.local) 
        }
    }
  }

  pub fn get_bind_group(&self,name_index:&UBONameIndex,m_eid:Option<u32>) -> Option<&wgpu::BindGroup> {
    match name_index.0 {
      UBOType::Component => {
        let array = &self.components[name_index.1];
        let eid = m_eid.log_err("not found eid in buffer")?;
        let bind_group = array.get_item(eid).map(|v| &v.bind_group);
        if bind_group.is_none() {
          if let Some(info) = self.comp_nameidxs.iter().filter(|v| v.1.1 == name_index.1).next() {
            log::error!("bind_group is none {:?} {}",m_eid,info.0.as_str());
          } else {
            log::error!("bind_group is none {:?}",m_eid);
          }
        }
        bind_group
      },
      UBOType::Global => { 
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

