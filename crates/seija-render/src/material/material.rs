use std::sync::Arc;
use super::{MaterialDef, RenderOrder};
use lite_clojure_eval::EvalRT;
use seija_core::{TypeUuid, bytes::Bytes};
use wgpu::{BufferUsage, Device};
use wgpu::{util::DeviceExt};
use crate::pipeline::render_bindings::{RenderBindGroup, RenderBindGroupLayout};
use crate::resource::{BufferId, RenderResourceId, RenderResources};
use crate::{material::read_material_def, memory::{TypedUniformBuffer}};
use uuid::Uuid;

#[derive(Debug,TypeUuid)]
#[uuid = "9fb83fbe-b850-42e0-a58c-53da87bace04"]
pub struct Material {
    pub def:Arc<MaterialDef>,
    pub order:RenderOrder,
    pub props:TypedUniformBuffer,
    pub buffer:Option<BufferId>,
    pub bind_group:Option<RenderBindGroup>
}

impl Material {
    pub fn from_def(def:Arc<MaterialDef>) -> Material {
        let props = TypedUniformBuffer::from_def(def.prop_def.clone());
        Material {
            order:def.order,
            def,
            props,
            buffer:None,
            bind_group:None
        }
    }

    pub fn check_create(&mut self,resources:&mut RenderResources,device:&Device,mat_layout:&Arc<RenderBindGroupLayout>) {
        if self.buffer.is_none() {
            let buffer = resources.create_buffer_with_data(BufferUsage::COPY_DST | BufferUsage::UNIFORM, self.props.get_buffer());
            self.buffer = Some(buffer.clone());
            let mut bind_group = RenderBindGroup::from_layout(mat_layout);
            bind_group.values.add(RenderResourceId::Buffer(buffer));
            bind_group.build(device, resources);
            self.bind_group = Some(bind_group);
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
    
    let mat = Material::from_def(Arc::new(material_def));
    let s = mat.props.get_f32("scale", 0);
    println!(" {}",s);
}