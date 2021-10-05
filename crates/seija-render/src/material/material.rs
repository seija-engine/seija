use std::collections::HashMap;
use super::{MaterialDef, RenderOrder};
use lite_clojure_eval::EvalRT;
use seija_core::{TypeUuid, bytes::Bytes};
use wgpu::{BufferUsage, Device};
use wgpu::{util::DeviceExt};
use crate::{material::read_material_def, memory::{TypedUniformBuffer, UniformBuffer}, render::AppRender};
use uuid::Uuid;

#[derive(Debug,TypeUuid)]
#[uuid = "9fb83fbe-b850-42e0-a58c-53da87bace04"]
pub struct Material {
    pub order:RenderOrder,
    pub props:TypedUniformBuffer,
    pub buffer:Option<wgpu::Buffer>
}

impl Material {
    pub fn from_def(def:&MaterialDef) -> Material {
        let props = TypedUniformBuffer::from_def(def.prop_def.clone());
        Material {
            order:def.order,
            props,
            buffer:None
        }
    }

    pub fn check_create(&mut self,device:&Device) {
     
        if self.buffer.is_none() {
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                contents:self.props.get_buffer(),
                label:None,
                usage:BufferUsage::COPY_DST | BufferUsage::UNIFORM,
            });
            self.buffer = Some(buffer);
            self.props.clear_dirty();
        }
        
    }

}



#[test]
fn test_material() {
    let test_md_string = r#"
    {
        :name "ui-color"
        :order "Transparent"
        :props [
            {:name "scale" :type "float" :default 3.14159265358},
            {:name "width" :type "int" :default 123456789},
            {:name "isMask" :type "bool" :default true},
        ]
        :pass {
            :z-write true
            :z-test "<"
            :cull "Back"
            :vs "ui.vert"
            :fs "ui.frag"
        }
    }
    "#;
    let mut vm = EvalRT::new();
    let material_def = read_material_def(&mut vm, &test_md_string).unwrap();
    
    let mut mat = Material::from_def(&material_def);
    let s = mat.props.get_f32("scale", 0);
    println!(" {}",s);
}