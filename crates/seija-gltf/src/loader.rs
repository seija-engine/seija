use crate::{GltfError, asset::{GltfAsset,GltfMesh,GltfPrimitive}};
use seija_asset::{Assets};
use seija_render::{resource::{Indices, Mesh, MeshAttributeType, VertexAttributeValues}, wgpu::{PrimitiveTopology}};

type ImportData = (gltf::Document, Vec<gltf::buffer::Data>, Vec<gltf::image::Data>);

pub fn load_gltf(path:&str,mesh_assets:&mut Assets<Mesh>) -> Result<GltfAsset,GltfError> {
    let import_data:ImportData = gltf::import(path).map_err(GltfError::LoadGltfError)?;
    let meshs = load_meshs(&import_data,mesh_assets)?;
    Ok(GltfAsset {
        meshs
    })
}

fn load_meshs(gltf:&ImportData,mesh_assets:&mut Assets<Mesh>) -> Result<Vec<GltfMesh>,GltfError> {
    let mut meshs:Vec<GltfMesh> = vec![];
    for mesh in gltf.0.meshes() {
        let mut primitives:Vec<GltfPrimitive> = vec![];
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&gltf.1[buffer.index()]));
            let primitive_topology = get_primitive_topology(primitive.mode())?;
            let mut mesh = Mesh::new(primitive_topology);
            if let Some(verts) = reader.read_positions().map(|iter| VertexAttributeValues::Float3(iter.collect())) {
                mesh.set(MeshAttributeType::POSITION, verts);
            }

            if let Some(normals) = reader.read_normals().map(|iter| VertexAttributeValues::Float3(iter.collect())) {
                mesh.set(MeshAttributeType::NORMAL, normals);
            }

            if let Some(uvs) = reader.read_tex_coords(0).map(|iter| VertexAttributeValues::Float2(iter.into_f32().collect())) {
                mesh.set(MeshAttributeType::UV0, uvs);
            }

            if let Some(uvs2) = reader.read_tex_coords(1).map(|iter| VertexAttributeValues::Float2(iter.into_f32().collect())) {
                mesh.set(MeshAttributeType::UV1, uvs2);
            }

            if let Some(tangents) = reader.read_tangents().map(|iter| VertexAttributeValues::Float4(iter.collect())) {
                mesh.set(MeshAttributeType::TANGENT, tangents);
            }

            if let Some(colors) = reader.read_colors(0).map(|iter| VertexAttributeValues::Float4(iter.into_rgba_f32().collect())) {
                mesh.set(MeshAttributeType::COLOR, colors);
            }


            if let Some(indices) = reader.read_indices() {
                mesh.set_indices(Some(Indices::U32(indices.into_u32().collect())));
            };
            mesh.build();
            let mesh_handle = mesh_assets.add(mesh);
            primitives.push(GltfPrimitive { mesh: mesh_handle });
        }
        meshs.push(GltfMesh { primitives });
    }

    Ok(meshs)
}

fn get_primitive_topology(mode: gltf::mesh::Mode) -> Result<PrimitiveTopology, GltfError> {
    match mode {
        gltf::mesh::Mode::Points => Ok(PrimitiveTopology::PointList),
        gltf::mesh::Mode::Lines => Ok(PrimitiveTopology::LineList),
        gltf::mesh::Mode::LineStrip => Ok(PrimitiveTopology::LineStrip),
        gltf::mesh::Mode::Triangles => Ok(PrimitiveTopology::TriangleList),
        gltf::mesh::Mode::TriangleStrip => Ok(PrimitiveTopology::TriangleStrip),
        mode => Err(GltfError::UnsupportedPrimitive(mode)),
    }
}