use std::{path::Path, sync::Arc};
use crate::anim_loader::load_skeleton;
use crate::{ImportData};
use crate::{GltfError, asset::{GltfAsset, GltfCamera, GltfMaterial, GltfMesh, GltfNode, GltfPrimitive, GltfScene, NodeIndex}};
use seija_asset::{Assets, Handle};
use seija_render::{camera::camera::{Orthographic, Perspective, Projection}, resource::{Indices, Mesh,Texture, MeshAttributeType, VertexAttributeValues}, wgpu, wgpu::{PrimitiveTopology}};
use seija_transform::{Transform, TransformMatrix};



pub fn load_gltf<P>(path:P,mesh_assets:&mut Assets<Mesh>,texture_assets:&mut Assets<Texture>) -> Result<GltfAsset,GltfError> where P:AsRef<Path> {
    let path:&Path = path.as_ref();
    let import_data:ImportData = gltf::import(path).map_err(GltfError::LoadGltfError)?;
    let textures = load_textures(&import_data,path,texture_assets)?;
    let materials = load_materials(&import_data,&textures)?;
    let meshs = load_meshs(&import_data,mesh_assets,&materials)?;
    let mut nodes = load_nodes(&import_data)?;
    let scenes = load_scenes(&import_data,&mut nodes)?;
    let a = load_skeleton(&import_data);
    Ok(GltfAsset {
        scenes,
        meshs,
        textures,
        materials,
        nodes
    })
}

fn load_nodes(gltf:&ImportData) -> Result<Vec<GltfNode>,GltfError> {
    let mut nodes:Vec<GltfNode> = vec![];
    for node in gltf.0.nodes() {
       let mesh = node.mesh().map(|m| m.index());
       let transform = match node.transform() {
           gltf::scene::Transform::Matrix {matrix} => {
             Transform::from_matrix(glam::Mat4::from_cols_array_2d(&matrix))
            },
           gltf::scene::Transform::Decomposed {translation,scale,rotation} => {
             Transform::new(glam::Vec3::from(translation), glam::Quat::from_array(rotation), glam::Vec3::from(scale))
           }
       };
       let camera = if let Some(camera) = node.camera() {
            Some(match camera.projection() {
                gltf::camera::Projection::Orthographic(o) => {
                    let xmag = o.xmag();
                    let ymag = o.ymag();
                    Projection::Ortho(Orthographic {
                        left:-xmag,
                        right: xmag,
                        top: ymag,
                        bottom: -ymag,
                        far: o.zfar(),
                        near: o.znear(),
                        ..Default::default()
                    })
                },
                gltf::camera::Projection::Perspective(p) => {
                    Projection::Perspective(Perspective {
                        fov: p.yfov(),
                        near: p.znear(),
                        ..Default::default()
                    })
                }
            })
       } else { None }.map(|p| GltfCamera {projection:p});

       nodes.push(GltfNode {
           camera,
           mesh, 
           children:vec![],
           transform
       });
    }
    Ok(nodes)
}

fn load_scenes(gltf:&ImportData,nodes:&mut Vec<GltfNode>) -> Result<Vec<GltfScene>,GltfError> {
    let mut scenes = vec![];
   
    for scene in gltf.0.scenes() {
        let node_indexs = scene.nodes().map(|n| n.index() ).collect();
        scenes.push(GltfScene { nodes:node_indexs });
        for node in scene.nodes() {
            load_node(&node,nodes,&TransformMatrix::default());
        }
    }
    Ok(scenes)
}

fn load_node(node:&gltf::Node,nodes:&mut Vec<GltfNode>,p_t:&TransformMatrix) {
    let cur_mat = p_t.mul_transform(&nodes[node.index()].transform.local);
    nodes[node.index()].transform.set_global(cur_mat.clone());
    let mut childrens:Vec<NodeIndex> = vec![];
    for child in node.children() {
        load_node(&child, nodes,&cur_mat);
        childrens.push(child.index());
    }
    nodes[node.index()].children = childrens;
}

fn load_textures(gltf:&ImportData,path:&Path,texture_assets:&mut Assets<Texture>) -> Result<Vec<Handle<Texture>>,GltfError> {
    let mut textures:Vec<Handle<Texture>> = vec![];
    for json_texture in gltf.0.textures() {
        let source = json_texture.source().source();
        match source {
            gltf::image::Source::View { view, mime_type:_ } => {
                let start = view.offset() as usize;
                let end = (view.offset() + view.length()) as usize;
                let buffer = &gltf.1[view.buffer().index()][start..end];
                let texture = Texture::from_bytes(buffer,None).map_err(|_| GltfError::LoadImageError)?;
                textures.push(texture_assets.add(texture));
            },
            gltf::image::Source::Uri { uri, mime_type:_ } => {
                let texture_path = path.parent().map(|p| p.join(uri)).ok_or(GltfError::LoadImageError)?;
                let bytes = std::fs::read(texture_path).map_err(|_| GltfError::LoadImageError)?;
                let mut texture = Texture::from_bytes(&bytes,None).map_err(|_| GltfError::LoadImageError)?;
                texture.sampler = get_texture_sampler(&json_texture);
                textures.push(texture_assets.add(texture));
            }
        }
    }
    Ok(textures)
}

fn load_materials(gltf:&ImportData,textures:&Vec<Handle<Texture>>) -> Result<Vec<Arc<GltfMaterial>>,GltfError> {
    let mut materials:Vec<Arc<GltfMaterial>> = vec![];
    for material in gltf.0.materials() {
        let pbr = material.pbr_metallic_roughness();
        let base_color_texture = if let Some(info) = pbr.base_color_texture() {
           Some(textures[info.texture().index()].clone())
        } else { None };
        

        materials.push(Arc::new(GltfMaterial {
            base_color:pbr.base_color_factor(),
            base_color_texture
        }));
    }
    Ok(materials)
}

fn load_meshs(gltf:&ImportData,mesh_assets:&mut Assets<Mesh>,materials:&Vec<Arc<GltfMaterial>>) -> Result<Vec<GltfMesh>,GltfError> {
    let mut meshs:Vec<GltfMesh> = vec![];
    for mesh in gltf.0.meshes() {
        let mut primitives:Vec<GltfPrimitive> = vec![];
        for primitive in mesh.primitives() {
            //dbg!(&primitive);
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

            let material = primitive.material().index().and_then(|idx| materials.get(idx)).map(|v|v.clone());
            primitives.push(GltfPrimitive { 
                mesh: mesh_handle ,
                material
            });
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

fn get_texture_sampler(texture: &gltf::Texture) -> wgpu::SamplerDescriptor<'static>  {
    let gltf_sampler = texture.sampler();
    
    wgpu::SamplerDescriptor { 
        label: None, 
        address_mode_u: texture_address_mode(&gltf_sampler.wrap_s()), 
        address_mode_v: texture_address_mode(&gltf_sampler.wrap_t()), 
        mag_filter: gltf_sampler.mag_filter().map(|mf| match mf {
            gltf::texture::MagFilter::Nearest => wgpu::FilterMode::Nearest,
            gltf::texture::MagFilter::Linear =>  wgpu::FilterMode::Linear,
        }).unwrap_or(wgpu::FilterMode::Nearest ), 
        min_filter: gltf_sampler
        .min_filter()
        .map(|mf| match mf {
              gltf::texture::MinFilter::Nearest
            | gltf::texture::MinFilter::NearestMipmapNearest
            | gltf::texture::MinFilter::NearestMipmapLinear => wgpu::FilterMode::Nearest,
              gltf::texture::MinFilter::Linear
            | gltf::texture::MinFilter::LinearMipmapNearest
            | gltf::texture::MinFilter::LinearMipmapLinear => wgpu::FilterMode::Linear,
        }).unwrap_or(wgpu::FilterMode::Linear), 
        mipmap_filter:  gltf_sampler
        .min_filter()
        .map(|mf| match mf {
              gltf::texture::MinFilter::Nearest
            | gltf::texture::MinFilter::Linear
            | gltf::texture::MinFilter::NearestMipmapNearest
            | gltf::texture::MinFilter::LinearMipmapNearest => wgpu::FilterMode::Nearest,
            gltf::texture::MinFilter::NearestMipmapLinear | gltf::texture::MinFilter::LinearMipmapLinear => {
                wgpu::FilterMode::Linear
            }
        })
        .unwrap_or(wgpu::FilterMode::Nearest),
        ..Default::default()
    }
}

fn texture_address_mode(gltf_address_mode: &gltf::texture::WrappingMode) -> wgpu::AddressMode {
    match gltf_address_mode {
        gltf::texture::WrappingMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
        gltf::texture::WrappingMode::Repeat => wgpu::AddressMode::Repeat,
        gltf::texture::WrappingMode::MirroredRepeat => wgpu::AddressMode::MirrorRepeat,
    }
}