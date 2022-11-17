use std::collections::HashMap;

use seija_asset::Handle;
use wgpu::CommandEncoder;

use crate::{UniformInfoSet, resource::{RenderResources, Texture}, memory::TypedUniformBuffer,};

use super::{object::UniformObject, array_object::ArrayObject, UniformType, UBOApplyType};

#[derive(Clone,Copy,Debug,Default)]
pub struct UniformIndex {
    pub typ:UniformType,
    pub index:usize,
    pub apply_type:UBOApplyType
}



#[derive(Default)]
pub struct UniformContext {
    pub info:UniformInfoSet,
    global_nameidxs:HashMap<String,UniformIndex>,
    //Name -> Object
    globals:Vec<UniformObject>,
    
    component_nameidxs:HashMap<String,UniformIndex>,
    //Name + Entity -> Object
    components:Vec<ArrayObject>,
}

impl UniformContext {
    pub fn init(&mut self,_:&mut RenderResources) {}

    pub fn add_uniform(&mut self,name:&str,res:&mut RenderResources) -> bool {
        if let Some(info) = self.info.globals.get(name) {
            let object = UniformObject::new(res, info);
            self.globals.push(object);
            self.global_nameidxs.insert(name.to_string(),
                                       UniformIndex {
                                           typ:UniformType::Global, 
                                           index:self.globals.len() - 1,
                                           apply_type:info.apply
                                      });
            return true;
        }

        if let Some(info) = self.info.components.get(name) {
            self.components.push(ArrayObject::new(info,res));
            self.component_nameidxs.insert(name.to_string(),
                                        UniformIndex { 
                                            typ:UniformType::Component, 
                                            index:self.components.len() - 1,
                                            apply_type:info.apply
                                          });
            return true;
        }
        false
    }

    pub fn remove_uniform(&mut self,name:&str) {
        if let Some(index) = self.global_nameidxs.get(name).map(|v| v.index) {
            self.globals.remove(index);
            self.global_nameidxs.remove(name);
        }
        if let Some(index) = self.component_nameidxs.get(name).map(|v| v.index) {
            self.components.remove(index);
            self.component_nameidxs.remove(name);
        }
    }

    pub fn get_index(&self,name:&str) -> Option<UniformIndex> {
        if self.component_nameidxs.contains_key(name) {
            return self.component_nameidxs.get(name).map(|v|*v);
         } else {
           return self.global_nameidxs.get(name).map(|v| *v);
         }
    }

    pub fn set_buffer<F>(&mut self,index:&UniformIndex,eid:Option<u32>,set_fn:F) where F:FnOnce(&mut TypedUniformBuffer) {
        match index.typ {
            UniformType::Global => {
                let object = &mut self.globals[index.index];
                set_fn(&mut object.local_buffer);
            },
            UniformType::Component => {
                let object = &mut self.components[index.index];
                if let Some(item_object) = eid.and_then(|id| object.get_item_mut(id)) {
                    set_fn(&mut item_object.buffer);
                }
            },
        }
    }

    

    pub fn set_texture(&mut self,eid:Option<u32>,ubo_name:&str,texture_name:&str,texture:Handle<Texture>) -> Result<(),i32> {
        let index = self.get_index(ubo_name).ok_or(0)?;
        match index.typ {
            UniformType::Global => {
                let object = &mut self.globals[index.index];
                object.set_texture(texture_name, texture);
            },
            UniformType::Component => {
                let object = &mut self.components[index.index];
                if let Some(item_object) = eid.and_then(|id| object.get_item_mut(id)) {
                    item_object.set_texture(texture_name, texture);
                }
            },
        }
        Ok(())
    }

    

    pub fn add_component(&mut self,index:&UniformIndex,eid:u32,res:&mut RenderResources) {
        let array_object = &mut self.components[index.index];
        array_object.add_item(eid, res);
    }

    pub fn remove_component(&mut self,index:&UniformIndex,eid:u32) {
        let array_object = &mut self.components[index.index];
        array_object.remove_item(eid);
    }

    pub fn get_bind_group(&self,index:&UniformIndex,eid:Option<u32>) -> Option<&wgpu::BindGroup> {
        match index.typ {
            UniformType::Global => {
                let object = &self.globals[index.index];
                object.bind_group.as_ref()
            },
            UniformType::Component => {
                let object = &self.components[index.index];
                eid.and_then(|id| object.get_item(id))
                   .and_then(|v| v.bind_group.as_ref())
            },
        }
    }

    pub fn set_bind_group(&mut self,index:&UniformIndex,eid:Option<u32>,bind_group:wgpu::BindGroup) {
        match index.typ {
            UniformType::Global => {
                let object = &mut self.globals[index.index];
                object.bind_group = Some(bind_group);
            },
            UniformType::Component => {
                let array_object = &mut self.components[index.index];
                if let Some(eid) = eid {
                    if let Some(item) = array_object.get_item_mut(eid) {
                        item.bind_group =Some(bind_group);
                    }
                }
            },
        }
    }

    pub fn get_layout(&self,name:&str) -> Option<&wgpu::BindGroupLayout> {
        let index = self.get_index(name)?;
        Some(self.get_layout_(&index))
    }

    pub fn get_layout_(&self,index:&UniformIndex) -> &wgpu::BindGroupLayout {
        match index.typ {
            UniformType::Global => {
                let object = &self.globals[index.index];
                &object.layout
            },
            UniformType::Component => {
                let object = &self.components[index.index];
                &object.layout
            },
        }
    }

    pub fn update(&mut self,res:&mut RenderResources,cmd:&mut CommandEncoder) {
        for global in self.globals.iter_mut() {
            global.update(res,cmd);
        }
        for comps in self.components.iter_mut() {
            comps.update(res,cmd);
        } 
    }
}