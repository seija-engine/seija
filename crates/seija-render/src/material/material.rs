use std::sync::Arc;
use super::{MaterialDef, RenderOrder};

use seija_asset::Handle;
use seija_core::{TypeUuid};
use wgpu::{BufferUsage, Device};
use crate::pipeline::render_bindings::{BindGroupBuilder, BindGroupLayoutBuilder};
use crate::resource::{BufferId, RenderResources, Texture};
use crate::{material::read_material_def, memory::{TypedUniformBuffer}};
use uuid::Uuid;

#[derive(Debug,TypeUuid)]
#[uuid = "9fb83fbe-b850-42e0-a58c-53da87bace04"]
pub struct Material {
    pub def:Arc<MaterialDef>,
    pub order:RenderOrder,
    pub props:TypedUniformBuffer,
    pub texture_props:TextureProps,
    pub buffer:Option<BufferId>,
    pub bind_group:Option<wgpu::BindGroup>
}

impl Material {
    pub fn from_def(def:Arc<MaterialDef>) -> Material {
        let props = TypedUniformBuffer::from_def(def.prop_def.clone());
        let texture_props = TextureProps::from_def(&def);
        Material {
            order:def.order,
            def,
            props,
            buffer:None,
            bind_group:None,
            texture_props
        }
    }

    pub fn is_ready(&self,resources:&RenderResources) -> bool {
       self.texture_props.is_ready(resources)
    }

    pub fn update(&mut self,resources:&mut RenderResources,device:&Device,mat_layout:&wgpu::BindGroupLayout,texture_layout:Option<&wgpu::BindGroupLayout>) {
        if self.buffer.is_none() {
            let buffer = resources.create_buffer_with_data(BufferUsage::COPY_DST | BufferUsage::UNIFORM, self.props.get_buffer());
            self.buffer = Some(buffer.clone());
            let mut bind_group_builder = BindGroupBuilder::new();
            bind_group_builder.add_buffer(buffer);
            self.bind_group = Some(bind_group_builder.build(mat_layout, device, resources) );
            self.props.clear_dirty();
        }
        self.texture_props.update(resources,device,texture_layout);
        
    }
}

#[derive(Debug)] 
pub struct TextureProps {
    is_dirty:bool,
    def:Arc<MaterialDef>,
    pub textures:Vec<Option<Handle<Texture>>>,
    pub bind_group:Option<wgpu::BindGroup>
}

impl TextureProps {
    pub fn from_def(def:&Arc<MaterialDef>) -> TextureProps {
        let mut textures:Vec<Option<Handle<Texture>>> = Vec::with_capacity(def.texture_idxs.len());
        for _ in 0..def.texture_idxs.len() {
            textures.push(None);
        }
        TextureProps {
            is_dirty:false,
            def:def.clone(),
            textures,
            bind_group:None
        }
    }

    pub fn is_dirty(&self) -> bool { self.is_dirty }

    pub fn set(&mut self,name:&str,texture:Handle<Texture>) {
        if let Some(index) = self.def.texture_idxs.get(name) {

            if let Some(old_texture) = &self.textures[*index] {
               if !self.is_dirty && old_texture.id != texture.id {
                   self.is_dirty = true;
               }
            } else {
                self.is_dirty = true;
            }
            
            self.textures[*index] = Some(texture);
        }
    }

    pub fn is_ready(&self,resources:&RenderResources) -> bool {
        for tex in self.textures.iter() {
            if let Some(texture) = tex {
                if resources.get_render_resource(&texture.id, 0).is_none() {
                    return false;
                }
            } else {
                return false
            }
        }
        true
    }

    pub fn update(&mut self,resources:&mut RenderResources,device:&Device,texture_layout:Option<&wgpu::BindGroupLayout>) {
        if self.textures.is_empty() {
            return;
        }
        if self.is_dirty {
            let mut bind_group_builder = BindGroupBuilder::new();
            for texture in self.textures.iter() {
                bind_group_builder.add_texture(texture.as_ref().unwrap().clone_weak());
            }
            let bind_group = bind_group_builder.build(texture_layout.unwrap(), device, resources);
            self.bind_group = Some(bind_group);
            self.is_dirty = false;
        }
    }
}


#[test]
fn test_material() {
    use lite_clojure_eval::EvalRT;
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