use std::sync::Arc;
use super::{MaterialDef, RenderOrder, MaterialDefineAsset};

use bevy_ecs::prelude::{Component, World};
use seija_asset::{Handle, AssetServer, Assets};
use seija_core::{TypeUuid};
use crate::pipeline::render_bindings::{BindGroupBuilder};
use crate::resource::{RenderResources, Texture};
use crate::{memory::{TypedUniformBuffer}};
use uuid::Uuid;

#[derive(Debug,TypeUuid,Component)]
#[uuid = "9fb83fbe-b850-42e0-a58c-53da87bace04"]
pub struct Material {
    pub define:Option<Handle<MaterialDefineAsset>>,
    pub def:Arc<MaterialDef>,
    pub order:RenderOrder,
    pub props:TypedUniformBuffer,
    pub texture_props:TextureProps,
    pub bind_group:Option<wgpu::BindGroup>
}

impl Material {

    pub fn from_def(def:Arc<MaterialDef>,server:&AssetServer) -> Option<Material> {
        let props = TypedUniformBuffer::from_def(def.prop_def.clone());
        let texture_props = TextureProps::from_def_new(&def,server)?;
        Some(Material {
            define:None,
            order:def.order,
            def,
            props,
            bind_group:None,
            texture_props
        })
    }

    pub fn from_world(world:&World,define:&str) -> Option<Material> {
        let server = world.get_resource::<AssetServer>()?;
        let h_define = server.get_asset(define)?.make_handle().typed::<MaterialDefineAsset>();
        let define = world.get_resource::<Assets<MaterialDefineAsset>>()?.get(&h_define.id)?.define.clone();
        Material::from_def(define, server)
    }

    pub fn is_ready(&self,resources:&RenderResources) -> bool {
       self.texture_props.is_ready(resources)
    }

 

}

#[derive(Debug)] 
pub struct TextureProps {
    is_dirty:bool,
    def:Arc<MaterialDef>,
    pub textures:Vec<Handle<Texture>>,
    pub bind_group:Option<wgpu::BindGroup>
}

impl TextureProps {
   
    pub fn from_def_new(def:&Arc<MaterialDef>,server:&AssetServer) -> Option<TextureProps> {
        let mut textures:Vec<Handle<Texture>> = Vec::with_capacity(def.tex_prop_def.indexs.len());
        for (_,info) in def.tex_prop_def.indexs.iter() {
            let handle = server.get_asset(info.def_asset.as_str())?;
            textures.push(handle.make_weak_handle().typed().clone_weak());
        }
        Some(TextureProps {
            is_dirty:true,
            def:def.clone(),
            textures,
            bind_group:None
        })
    }

    pub fn is_dirty(&self) -> bool { self.is_dirty }

    pub fn set(&mut self,name:&str,texture:Handle<Texture>) {
        if let Some(v) = self.def.tex_prop_def.indexs.get(name) {
            let old_texture = &self.textures[v.index];
            if !self.is_dirty && old_texture.id != texture.id {
                self.is_dirty = true;
            }
            self.textures[v.index] = texture;
        }
    }

    pub fn is_ready(&self,resources:&RenderResources) -> bool {
        if self.textures.is_empty() { return  true; }
        for tex in self.textures.iter() {
            if resources.get_render_resource(&tex.id, 0).is_none() {      
                return false;
            }
        }
        true
    }

    pub fn update(&mut self,resources:&mut RenderResources,texture_layout:Option<&wgpu::BindGroupLayout>) {
        if self.textures.is_empty() {
            return;
        }
        if self.is_dirty && self.textures.len() > 0 {
            let mut bind_group_builder = BindGroupBuilder::new();
            for texture in self.textures.iter() {
                bind_group_builder.add_texture(texture.clone_weak());
            }
            let bind_group = bind_group_builder.build(texture_layout.unwrap(), &resources.device, resources);
            self.bind_group = Some(bind_group);
            self.is_dirty = false;
        }
    }
}

