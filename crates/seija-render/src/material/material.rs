use std::collections::HashMap;

use super::{MaterialDef, RenderOrder, material_def::PropDef};
use lite_clojure_eval::EvalRT;
use seija_core::{TypeUuid, bytes::Bytes};
use crate::material::read_material_def;
use uuid::Uuid;

#[derive(Debug,TypeUuid)]
#[uuid = "9fb83fbe-b850-42e0-a58c-53da87bace04"]
pub struct Material {
    pub order:RenderOrder,
    props:MaterialPropBytes
}

impl Material {
    pub fn from_def(def:&MaterialDef) -> Material {
        let byte_props = MaterialPropBytes::from_props(&def.prop_defs);
        Material {
            order:def.order,
            props:byte_props
        }
    }
}

#[derive(Debug)]
pub struct MaterialPropBytes {
    bytes:Vec<u8>,
    name_offset:HashMap<String,usize>
}

impl MaterialPropBytes {
    pub fn get_u8(&self,name:&str) -> Option<u8> {
        if let Some(v) = self.name_offset.get(name) {
            return  Some(self.bytes[*v]) 
        }
        None
    }

    pub fn set_u8(&mut self,name:&str,value:u8) {
        if let Some(v) = self.name_offset.get(name) {
            self.bytes[*v] = value
        }
    }

    pub fn get_bool(&self,name:&str) -> Option<bool> {
        self.get_u8(name).map(|v| v > 0)
    }

    pub fn set_bool(&mut self,name:&str,b:bool)  {
        self.set_u8(name,if b {1} else {0})
    }

    pub fn get_i32(&self,name:&str) -> Option<i32> {
        if let Some(v) = self.name_offset.get(name) {
            let offset = *v;
            let mut bytes:[u8;4] = [0,0,0,0];
            let slice = &self.bytes[offset..(offset + 4)];
            bytes.copy_from_slice(slice);
            return Some(i32::from_be_bytes(bytes));
        }
        None
    }

    pub fn set_i32(&mut self,name:&str,value:i32)  {
        if let Some(v) = self.name_offset.get(name) {
            let bytes:[u8;4] = i32::to_be_bytes(value);
            self.bytes[*v..(*v + 4)].copy_from_slice(&bytes);
        }
    }

    pub fn set_f32(&mut self,name:&str,value:f32)  {
        if let Some(v) = self.name_offset.get(name) {
            let bytes = f32::to_be_bytes(value);
            self.bytes[*v..(*v + 4)].copy_from_slice(&bytes);
        }
    }

    pub fn get_f32(&self,name:&str) -> Option<f32> {
        if let Some(v) = self.name_offset.get(name) {
            let offset = *v;
            let mut bytes:[u8;4] = [0,0,0,0];
            let slice = &self.bytes[offset..(offset + 4)];
            bytes.copy_from_slice(slice);
            return Some(f32::from_be_bytes(bytes));
        }
        None
    }

    pub fn from_props(props:&Vec<PropDef>) -> MaterialPropBytes {
        let mut name_offset:HashMap<String,usize> = HashMap::new();
        let mut byte_len = 0;
        for prop in props.iter() {
            byte_len += prop.value.byte_len();
        }
        let mut buffer:Vec<u8> = vec![0;byte_len];
        let buffer_slice = buffer.as_mut_slice();
        
        let mut offset = 0;
        for prop in props.iter() {
            name_offset.insert(prop.name.to_string(), offset);
            prop.value.write_bytes(&mut buffer_slice[offset..]);
            offset += prop.value.byte_len();
        }
        MaterialPropBytes {
            bytes:buffer,
            name_offset
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
            {:name "scale" :type "Float" :default 3.14159265358},
            {:name "width" :type "Int" :default 123456789},
            {:name "isMask" :type "Bool" :default true},
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
    mat.props.set_i32("width", 666666666);
    mat.props.set_f32("scale", 0.3333f32);
    let num = mat.props.get_i32("width");
    let fnum = mat.props.get_f32("scale");
    let is_mask = mat.props.get_bool("isMask");
    dbg!(num);
    dbg!(fnum);
    dbg!(is_mask);
}