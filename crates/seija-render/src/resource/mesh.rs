use std::{collections::HashSet, hash::{Hash, Hasher}};
use bevy_ecs::prelude::*;
use fnv::FnvHasher;
use seija_asset::{AssetEvent, Assets, Handle, HandleUntyped};
use seija_core::{bytes::AsBytes, event::{EventReader, Events, ManualEventReader}};
use wgpu::{BufferUsage, PrimitiveTopology, VertexFormat};
use seija_core::TypeUuid;
use uuid::Uuid;

use crate::{render::RenderContext, resource::RenderResourceId};

use super::shape;

#[derive(Debug,TypeUuid)]
#[uuid = "ea48c171-e7b4-4e54-8895-dda5a2d0fa90"]
pub struct Mesh {
    typ:PrimitiveTopology,
    values:Vec<VertexAttributeValues>,
    indices:Option<Indices>
}

impl Mesh {
    fn fnv_hash_u64(&self) -> u64 {
        let mut fnv_hasher = FnvHasher::default();
        self.hash(&mut fnv_hasher);
        fnv_hasher.finish()
    }
}

impl Hash for Mesh {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.typ.hash(state);
        core::mem::discriminant(&self.values).hash(state);
        if let Some(idxs) = self.indices.as_ref() {
            1.hash(state);
            core::mem::discriminant(idxs).hash(state);
        } else {
            0.hash(state);
        }
    }
}

#[derive(Debug, Clone)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

#[derive(Clone, Debug)]
pub enum VertexAttributeValues {
    Float(Vec<f32>),
    Int(Vec<i32>),
    UInt(Vec<u32>),
    Float2(Vec<[f32;2]>),
    Int2(Vec<[i32;2]>),
    UInt2(Vec<[u32;2]>),
    Float3(Vec<[f32;3]>),
    Int3(Vec<[i32;3]>),
    UInt3(Vec<[u32;3]>),
    Float4(Vec<[f32;4]>),
    Int4(Vec<[i32;4]>),
    UInt4(Vec<[u32;4]>),
    U84(Vec<[u8; 4]>),
}


impl VertexAttributeValues {
    pub fn get_bytes(&self) -> &[u8] {
        match self {
            VertexAttributeValues::Float(values) => values.as_slice().as_bytes(),
            VertexAttributeValues::Int(values) => values.as_slice().as_bytes(),
            VertexAttributeValues::UInt(values) => values.as_slice().as_bytes(),
            VertexAttributeValues::Float2(values) => values.as_slice().as_bytes(),
            VertexAttributeValues::Int2(values) => values.as_slice().as_bytes(),
            VertexAttributeValues::UInt2(values) => values.as_slice().as_bytes(),
            VertexAttributeValues::Float3(values) => values.as_slice().as_bytes(),
            VertexAttributeValues::Int3(values) => values.as_slice().as_bytes(),
            VertexAttributeValues::UInt3(values) => values.as_slice().as_bytes(),
            VertexAttributeValues::Float4(values) => values.as_slice().as_bytes(),
            VertexAttributeValues::Int4(values) => values.as_slice().as_bytes(),
            VertexAttributeValues::UInt4(values) => values.as_slice().as_bytes(),
            VertexAttributeValues::U84(values) => values.as_slice().as_bytes(),
        }
    }

    pub fn len(&self) -> usize {
        match *self {
            VertexAttributeValues::Float(ref values) => values.len(),
            VertexAttributeValues::Int(ref values) => values.len(),
            VertexAttributeValues::UInt(ref values) => values.len(),
            VertexAttributeValues::Float2(ref values) => values.len(),
            VertexAttributeValues::Int2(ref values) => values.len(),
            VertexAttributeValues::UInt2(ref values) => values.len(),
            VertexAttributeValues::Float3(ref values) => values.len(),
            VertexAttributeValues::Int3(ref values) => values.len(),
            VertexAttributeValues::UInt3(ref values) => values.len(),
            VertexAttributeValues::Float4(ref values) => values.len(),
            VertexAttributeValues::Int4(ref values) => values.len(),
            VertexAttributeValues::UInt4(ref values) => values.len(),
            VertexAttributeValues::U84(ref values) => values.len(),
        }
    }
}

impl From<&VertexAttributeValues> for VertexFormat {
    fn from(values: &VertexAttributeValues) -> Self {
        match values {
            VertexAttributeValues::Float(_) => VertexFormat::Float32,
            VertexAttributeValues::Int(_) => VertexFormat::Sint32,
            VertexAttributeValues::UInt(_) => VertexFormat::Uint32,
            VertexAttributeValues::Float2(_) => VertexFormat::Float32x2,
            VertexAttributeValues::Int2(_) => VertexFormat::Sint32x2,
            VertexAttributeValues::UInt2(_) => VertexFormat::Uint32x2,
            VertexAttributeValues::Float3(_) => VertexFormat::Float32x3,
            VertexAttributeValues::Int3(_) => VertexFormat::Sint32x3,
            VertexAttributeValues::UInt3(_) => VertexFormat::Uint32x3,
            VertexAttributeValues::Float4(_) => VertexFormat::Float32x4,
            VertexAttributeValues::Int4(_) => VertexFormat::Sint32x4,
            VertexAttributeValues::UInt4(_) => VertexFormat::Uint32x4,
            VertexAttributeValues::U84(_) => VertexFormat::Unorm8x4,
        }
    }
}

impl From<Vec<f32>> for VertexAttributeValues {
    fn from(vec: Vec<f32>) -> Self {
        VertexAttributeValues::Float(vec)
    }
}

impl From<Vec<i32>> for VertexAttributeValues {
    fn from(vec: Vec<i32>) -> Self {
        VertexAttributeValues::Int(vec)
    }
}

impl From<Vec<u32>> for VertexAttributeValues {
    fn from(vec: Vec<u32>) -> Self {
        VertexAttributeValues::UInt(vec)
    }
}

impl From<Vec<[f32; 2]>> for VertexAttributeValues {
    fn from(vec: Vec<[f32; 2]>) -> Self {
        VertexAttributeValues::Float2(vec)
    }
}

impl From<Vec<[i32; 2]>> for VertexAttributeValues {
    fn from(vec: Vec<[i32; 2]>) -> Self {
        VertexAttributeValues::Int2(vec)
    }
}

impl From<Vec<[u32; 2]>> for VertexAttributeValues {
    fn from(vec: Vec<[u32; 2]>) -> Self {
        VertexAttributeValues::UInt2(vec)
    }
}

impl From<Vec<[f32; 3]>> for VertexAttributeValues {
    fn from(vec: Vec<[f32; 3]>) -> Self {
        VertexAttributeValues::Float3(vec)
    }
}

impl From<Vec<[i32; 3]>> for VertexAttributeValues {
    fn from(vec: Vec<[i32; 3]>) -> Self {
        VertexAttributeValues::Int3(vec)
    }
}

impl From<Vec<[u32; 3]>> for VertexAttributeValues {
    fn from(vec: Vec<[u32; 3]>) -> Self {
        VertexAttributeValues::UInt3(vec)
    }
}

impl From<Vec<[f32; 4]>> for VertexAttributeValues {
    fn from(vec: Vec<[f32; 4]>) -> Self {
        VertexAttributeValues::Float4(vec)
    }
}

impl From<Vec<[i32; 4]>> for VertexAttributeValues {
    fn from(vec: Vec<[i32; 4]>) -> Self {
        VertexAttributeValues::Int4(vec)
    }
}

impl From<Vec<[u32; 4]>> for VertexAttributeValues {
    fn from(vec: Vec<[u32; 4]>) -> Self {
        VertexAttributeValues::UInt4(vec)
    }
}

impl From<Vec<[u8; 4]>> for VertexAttributeValues {
    fn from(vec: Vec<[u8; 4]>) -> Self {
        VertexAttributeValues::U84(vec)
    }
}

impl Mesh {
    pub fn new(typ:PrimitiveTopology) -> Mesh {
        Mesh {
            typ,
            values:Vec::new(),
            indices:None
        }
    }
    pub fn typ(&self) -> PrimitiveTopology {
        self.typ
    }

    pub fn add_value(&mut self,value:impl Into<VertexAttributeValues>) {
        self.values.push(value.into());
    }

    pub fn set_indices(&mut self,indices:Option<Indices>) {
        self.indices = indices
    }

    pub fn get_index_buffer_bytes(&self) -> Option<Vec<u8>> {
        self.indices.as_ref().map(|idxs| {
            match &idxs {
                Indices::U16(v) => v.as_slice().as_bytes().to_vec(),
                Indices::U32(v) => v.as_slice().as_bytes().to_vec(),
            }
        })
    }

    pub fn count_vertices(&self) -> usize { 
        self.values.first().map(|v| v.len()).unwrap_or(0) 
    }

    pub fn get_vertex_buffer_data(&self) -> Vec<u8> {
        let mut vert_size:usize = 0;
        for value in self.values.iter() {
            let format:VertexFormat = VertexFormat::from(value);
            vert_size += format.size() as usize;
        }
        let vertex_count = self.count_vertices();
        let mut buffer = vec![0; vertex_count * vert_size];
        let mut value_offset = 0;
        for value in self.values.iter() {
            let format = VertexFormat::from(value);
            let size = format.size() as usize;
            let bytes = value.get_bytes();
            for (vertex_index, chunk_bytes) in bytes.chunks_exact(size).enumerate() {
                let offset = vertex_index * vert_size + value_offset;
                buffer[offset..offset + size].copy_from_slice(chunk_bytes);
            }
            value_offset += size;
        }
        buffer
    }
}



pub fn update_mesh_system(world:&mut World,mesh_reader:&mut ManualEventReader<AssetEvent<Mesh>>,ctx:&mut RenderContext) {
    let mut changed_meshes:HashSet<Handle<Mesh>> = HashSet::default();
    {
        if let Some(mesh_events) = world.get_resource::<Events<AssetEvent<Mesh>>>() {
            for event in mesh_reader.iter(mesh_events) {
                match event {
                    AssetEvent::Created { ref handle } =>  {
                        changed_meshes.insert(handle.clone_weak());
                    }
                    AssetEvent::Modified { .. } => { }
                    AssetEvent::Removed { ref handle } =>  { 
                        changed_meshes.remove(handle);
                        remove_resource(handle.clone_weak_untyped(),0,ctx);
                        remove_resource(handle.clone_weak_untyped(),1,ctx);
                    }
                }
            }   
        }
        
    };
    let meshs = world.get_resource::<Assets<Mesh>>().unwrap();
    for mesh_handle in changed_meshes.iter() {
        if let Some(mesh) = meshs.get(&mesh_handle.id) {
            let vert_bytes = mesh.get_vertex_buffer_data();
            let vert_buffer = ctx.resources.create_buffer_with_data(BufferUsage::VERTEX, &vert_bytes);
            ctx.resources.set_render_resource(mesh_handle.clone_weak_untyped(), RenderResourceId::Buffer(vert_buffer), 0);

            if let Some(idx_bytes) = mesh.get_index_buffer_bytes() {
               let index_buffer = ctx.resources.create_buffer_with_data(BufferUsage::INDEX, &idx_bytes);
               ctx.resources.set_render_resource(mesh_handle.clone_weak_untyped(), RenderResourceId::Buffer(index_buffer), 1);
            }
        }
    }
}

fn remove_resource(handle:HandleUntyped,idx:u8,ctx:&mut RenderContext) {
    if let Some(RenderResourceId::Buffer(buffer)) = ctx.resources.get_render_resource(handle.clone(), idx) {
        ctx.resources.remove_buffer(buffer);
        ctx.resources.remove_render_resource(handle, idx);
    }
}