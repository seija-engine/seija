use seija_asset::AssetEvent;
use seija_core::{bytes::AsBytes, event::EventReader};
use wgpu::{PrimitiveTopology, VertexFormat};
use seija_core::TypeUuid;
use uuid::Uuid;

#[derive(Debug,TypeUuid)]
#[uuid = "58ee0320-a01e-4a1b-9d07-ade19767853b"]
pub struct Mesh {
    typ:PrimitiveTopology,
    values:Vec<VertexAttributeValues>,
    indices:Option<Indices>
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



pub(crate) fn update_mesh_system(mut mesh_events:EventReader<AssetEvent<Mesh>>) {
    for event in mesh_events.iter() {
        match event {
            AssetEvent::Created { ref handle } =>  { }
            AssetEvent::Modified { ref handle } => { }
            AssetEvent::Removed { ref handle } =>  { }
        }
    }
}