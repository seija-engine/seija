use std::{path::Path, sync::Arc, collections::HashMap, fmt::Debug};
use crate::{ImportData};
use glam::{Mat4, Vec3, Vec4};
use gltf::{Document, Node, animation::{Channel, Property, Interpolation}};
use seija_core::anyhow::{Result, anyhow};
use crate::{asset::{GltfAsset, GltfCamera, GltfMaterial, GltfMesh, GltfNode, GltfPrimitive, GltfScene, NodeIndex}};
use seija_asset::{Handle, AssetLoader, AssetServer, LoadingTrack, AssetLoaderParams, AssetDynamic};
use seija_render::resource::{Texture, TextureDescInfo};
use seija_render::{camera::camera::{Orthographic, Perspective, Projection}, 
                   resource::{Indices, Mesh, MeshAttributeType, VertexAttributeValues}, 
                   wgpu, wgpu::{PrimitiveTopology}};
use seija_skeleton3d::{Skeleton, AnimationSet, Skin, offine::{raw_skeleton::{RawSkeleton, RawJoint}, skeleton_builder::SkeletonBuilder, raw_animation::{RawAnimation, RawJointTrack, RawTranslationKey, RawScaleKey, RawRotationKey}, animation_builder::AnimationBuilder}, Animation};
use seija_transform::{Transform, TransformMatrix};
use async_trait::{async_trait};
pub struct GLTFLoader;

#[async_trait]
impl AssetLoader for GLTFLoader {
   async fn load(&self,server:AssetServer,track:Option<LoadingTrack>,path:&str,_:Option<Box<dyn AssetLoaderParams>>) -> Result<Box<dyn AssetDynamic>> {
       track.as_ref().map(|t| t.add_progress());
       let import_data:ImportData = gltf::import(path)?;
       track.as_ref().map(|t| t.add_progress());
       let textures = load_textures(&server, &import_data, path.as_ref())?;
       track.as_ref().map(|t| t.add_progress());
       let materials = load_materials(&import_data,&textures);
       track.as_ref().map(|t| t.add_progress());
       let mut meshs = load_meshs(&server,&import_data,&materials)?;
       track.as_ref().map(|t| t.add_progress());
       let mut nodes = load_nodes(&import_data)?;
       track.as_ref().map(|t| t.add_progress());
       let scenes = load_scenes(&import_data,&mut nodes,&mut meshs);
       track.as_ref().map(|t| t.add_progress());
       let _skeleton = load_skeleton(&import_data)?;
       track.as_ref().map(|t| t.add_progress());

       let mut skins = None;
       let mut anims = None;
       if let Some(skeleton) = _skeleton.as_ref() {
          skins = load_skin(&import_data, &skeleton).map(|v| server.create_asset(v,None));
          track.as_ref().map(|t| t.add_progress());
          let anim_set = load_animations(&import_data, &skeleton)?;
          track.as_ref().map(|t| t.add_progress());
          anims = Some(server.create_asset(anim_set,None)) ;
       }
       let skeleton = _skeleton.map(|v| server.create_asset(v,None));
       track.as_ref().map(|t| t.add_progress());
       Ok(Box::new(GltfAsset {
        scenes,
        meshs,
        textures,
        materials,
        nodes,
        skeleton,
        anims,
        skins
     }))
   }
}

fn load_textures(server:&AssetServer,gltf:&ImportData,path:&Path) -> Result<Vec<Handle<Texture>>> {
    let mut textures:Vec<Handle<Texture>> = vec![];
    for json_texture in gltf.0.textures() {
        let source = json_texture.source().source();
        match source {
            gltf::image::Source::View { view, mime_type:_ } => {
                let start = view.offset() as usize;
                let end = (view.offset() + view.length()) as usize;
                let buffer = &gltf.1[view.buffer().index()][start..end];
                let mut desc = TextureDescInfo::default();
                desc.sampler_desc = get_texture_sampler(&json_texture);
                let texture = Texture::from_image_bytes(buffer,desc)?;
                let h_texture = server.create_asset(texture,None);
                textures.push(h_texture);
            },
            gltf::image::Source::Uri { uri, mime_type:_ } => {
                let texture_path = path.parent().map(|p| p.join(uri)).ok_or(anyhow!("{}",uri))?;
                let bytes = std::fs::read(texture_path)?;
                let mut desc = TextureDescInfo::default();
                desc.sampler_desc = get_texture_sampler(&json_texture);
                let texture = Texture::from_image_bytes(&bytes,desc)?;
                let h_texture = server.create_asset(texture,None);
                textures.push(h_texture);
            }
        }
    }
    Ok(textures)
}


fn load_materials(gltf:&ImportData,textures:&Vec<Handle<Texture>>) -> Vec<Arc<GltfMaterial>> {
    let mut materials:Vec<Arc<GltfMaterial>> = vec![];
    for material in gltf.0.materials() {
        let pbr = material.pbr_metallic_roughness();
        let base_color_texture = if let Some(info) = pbr.base_color_texture() {
           Some(textures[info.texture().index()].clone())
        } else { None };
        
        let normal_texture:Option<Handle<Texture>> = if let Some(info) = material.normal_texture() {
            Some(textures[info.texture().index()].clone())
        } else { None };

        let metallic_roughness_texture:Option<Handle<Texture>> = if let Some(info) = pbr.metallic_roughness_texture() {
            Some(textures[info.texture().index()].clone())
        } else { None };

        let emissive_texture:Option<Handle<Texture>> = if let Some(info) = material.emissive_texture() {
            Some(textures[info.texture().index()].clone())
        } else { None };

        let metallic_factor = pbr.metallic_factor();
        let roughness_factor = pbr.roughness_factor();
        let emissive_factor =  Vec3::from(material.emissive_factor());
        let double_sided = material.double_sided();
        let alpha_cutoff = material.alpha_cutoff();
        let alpha_mode = material.alpha_mode();
        materials.push(Arc::new(GltfMaterial {
            base_color_factor:Vec4::from(pbr.base_color_factor()),
            base_color_texture,
            normal_texture,
            metallic_roughness_texture,
            emissive_texture,
            metallic_factor,
            roughness_factor,
            double_sided,
            alpha_cutoff,
            alpha_mode,
            emissive_factor
        }));
    }
    materials
}

fn load_meshs(server:&AssetServer,gltf:&ImportData,materials:&Vec<Arc<GltfMaterial>>) -> Result<Vec<GltfMesh>> {
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
            
            if let Some(joint0) = reader.read_joints(0).map(|iter|VertexAttributeValues::UInt16X4(iter.into_u16().collect())) {
                mesh.set(MeshAttributeType::JOINTS, joint0);
            }

            if let Some(weights) = reader.read_weights(0).map(|iter|VertexAttributeValues::Float4(iter.into_f32().collect())) {
                mesh.set(MeshAttributeType::WEIGHTS, weights);
            }


            if let Some(indices) = reader.read_indices() {
                mesh.set_indices(Some(Indices::U32(indices.into_u32().collect())));
            };
            mesh.build();
            let mesh_handle = server.create_asset::<Mesh>(mesh,None);
            let material = primitive.material().index().and_then(|idx| materials.get(idx)).map(|v|v.clone());
            primitives.push(GltfPrimitive { 
                mesh: mesh_handle ,
                material
            });
        }
        meshs.push(GltfMesh { node_index:0,primitives });
    }

    Ok(meshs)
}

fn load_nodes(gltf:&ImportData) -> Result<Vec<GltfNode>> {
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

fn load_scenes(gltf:&ImportData,nodes:&mut Vec<GltfNode>,meshs:&mut Vec<GltfMesh>) -> Vec<GltfScene> {
    let mut scenes = vec![];
   
    for scene in gltf.0.scenes() {
        let node_indexs = scene.nodes().map(|n| n.index() ).collect();
        scenes.push(GltfScene { nodes:node_indexs });
        for node in scene.nodes() {
            load_node(&node,nodes,&TransformMatrix::default(),meshs);
        }
    }
    scenes
}

fn load_node(node:&gltf::Node,nodes:&mut Vec<GltfNode>,p_t:&TransformMatrix,meshs:&mut Vec<GltfMesh>) {
    let cur_mat = p_t.mul_transform(&nodes[node.index()].transform.local);
    nodes[node.index()].transform.set_global(cur_mat.clone());
    if let Some(mesh) = node.mesh() {
        meshs[mesh.index()].node_index = node.index();
    }
    let mut childrens:Vec<NodeIndex> = vec![];
    for child in node.children() {
        load_node(&child, nodes,&cur_mat,meshs);
        childrens.push(child.index());
    }
    nodes[node.index()].children = childrens;
}


fn get_primitive_topology(mode: gltf::mesh::Mode) -> Result<PrimitiveTopology> {
    match mode {
        gltf::mesh::Mode::Points => Ok(PrimitiveTopology::PointList),
        gltf::mesh::Mode::Lines => Ok(PrimitiveTopology::LineList),
        gltf::mesh::Mode::LineStrip => Ok(PrimitiveTopology::LineStrip),
        gltf::mesh::Mode::Triangles => Ok(PrimitiveTopology::TriangleList),
        gltf::mesh::Mode::TriangleStrip => Ok(PrimitiveTopology::TriangleStrip),
        mode => Err(anyhow!("UnsupportedPrimitive:{:?}",mode)),
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

pub fn load_skeleton(gltf:&ImportData) -> Result<Option<Skeleton>> {
    let cur_scene = if let Some(scene) = gltf.0.default_scene() {
        scene
    } else { 
        if let Some(fst) = gltf.0.scenes().next() { fst } else { return Ok(None) }
    };
    if cur_scene.nodes().len() == 0 { return Ok(None) }
    let skins = get_skins_for_scene(&cur_scene,&gltf.0);
    let mut roots = if skins.len() == 0 {
        cur_scene.nodes().collect::<Vec<_>>()
    } else {
        find_skin_root_joint(&skins,&gltf.0)
    };
    roots.sort_by(|a,b|a.index().cmp(&b.index()));
    roots.dedup_by(|a,b|a.index() == b.index());

    let mut raw_skeleton:RawSkeleton = RawSkeleton::default();
    for root_node in roots {
        let joint = import_node_to_joint(&root_node);
        raw_skeleton.roots.push(joint);
    }

    let skeleton = SkeletonBuilder::build(&raw_skeleton);
    Ok(Some(skeleton))
}

fn get_skins_for_scene<'a>(scene:&gltf::Scene<'a>,doc:&'a gltf::Document) -> Vec<gltf::Skin<'a>> {
    let mut open:HashMap<usize,gltf::Node> = HashMap::default();
    let mut found:HashMap<usize,gltf::Node> = HashMap::default();
   
    for node in scene.nodes() {
        open.insert(node.index(),node);
    }

    while !open.is_empty() {
        let node = open.values().next().unwrap().clone();
        open.remove(&node.index());
        found.insert(node.index(),node.clone());
        
        for cnode in node.children() {
            open.insert(cnode.index(), cnode);
        }   
    }
    let mut skins:Vec<gltf::Skin<'a>> = vec![];
    for skin in doc.skins() {
        if let Some(fst) = skin.joints().next() {
            if found.contains_key(&fst.index()) {
                skins.push(skin);
            }
        }
    }
    skins
}

fn find_skin_root_joint<'a>(skins:&Vec<gltf::Skin<'a>>,doc:&'a Document) -> Vec<Node<'a>> {
    let mut roots:Vec<Node> = vec![];
    let mut parents:HashMap<usize,(u8,Option<Node>)> = HashMap::default();
    for node in doc.nodes() {
       parents.insert(node.index(), (0,None));
    }

    for node in doc.nodes() {
        for cnode in node.children() {
           let entry = parents.get_mut(&cnode.index()).unwrap();
           entry.0 = 1;
           entry.1 = Some(cnode)
        }
     }

    for skin in skins {
        if skin.joints().count() == 0 { continue; }
        if let Some(skeleton) = skin.skeleton() {
            let entry = parents.get_mut(&skeleton.index()).unwrap();
            entry.0 = 2;
            roots.push(entry.1.clone().unwrap());
        }

       
        if let Some((1,Some(n))) = parents.get(&skin.joints().next().unwrap().index()) {
            let mut root = n.clone();
            loop {
                match parents.get(&root.index()) {
                    Some((1,Some(n))) => {
                        root = n.clone();
                    },
                    _ => { break; }
                }
            }
            roots.push(root)
        }
    }
    roots
}


fn import_node_to_joint(node:&Node) -> RawJoint {
    let mut raw_joint= RawJoint::default();
    raw_joint.name = node.name().map(|v| v.to_string());
    
    let transform:TransformMatrix = match node.transform() {
        gltf::scene::Transform::Matrix {matrix} => {
            glam::Mat4::from_cols_array_2d(&matrix).into()
          
        },
        gltf::scene::Transform::Decomposed {translation,scale,rotation} => {
          TransformMatrix {position: glam::Vec3::from(translation),rotation: glam::Quat::from_array(rotation), scale: glam::Vec3::from(scale)}
        }
    };
    raw_joint.transform = transform;
    for cnode in node.children() {
        let joint = import_node_to_joint(&cnode);
        raw_joint.children.push(joint);
    }
    raw_joint
}

pub fn load_skin(gltf:&ImportData,skeleton:&Skeleton) -> Option<Skin> {
    let fst_skin = gltf.0.skins().next()?;
    let joint_count = fst_skin.joints().count();
    let mat4s = if let Some(inverse_mats) = fst_skin.inverse_bind_matrices() {
        let view = inverse_mats.view()?;
        let start = view.offset() + inverse_mats.offset();
        let end = start + (view.stride().unwrap_or(0) * inverse_mats.count());
        let buffer = &gltf.1[view.buffer().index()][start..end];
        let key_values:&[[f32;16]] =  unsafe { std::slice::from_raw_parts(buffer.as_ptr() as * const [f32;16], inverse_mats.count()) };
        let mats  = key_values.iter().map(Mat4::from_cols_array).collect::<Vec<_>>();
        mats
    } else {
        vec![Mat4::IDENTITY;joint_count]
    };
    let mut index = 0;
    for node in fst_skin.joints() {
        if node.name() != skeleton.joint_names[index].as_ref().map(|v| v.as_str()) {
            log::error!("skin joint sort error index:{}",index);
            return None;
        }
        index += 1;
    }
    
    Some(Skin::new(mat4s))
}

pub fn load_animations(data:&ImportData,skeleton:&Skeleton) -> Result<AnimationSet> {
    let mut anim_set = AnimationSet::default();
    for gltf_anim in data.0.animations() {
       let animation = import_animation(data,&gltf_anim, skeleton)?;
       anim_set.add(animation);
    }
    Ok(anim_set)
}

fn import_animation(data:&ImportData,animation:&gltf::Animation,skeleton:&Skeleton) -> Result<Animation> {
    let mut raw_animation = RawAnimation::default();
    raw_animation.name = animation.name().unwrap_or("none").to_string();
    raw_animation.duration = 0f32;
    let mut channels_per_joint:HashMap<&str,Vec<Channel>> = HashMap::default();
    for channel in animation.channels() {
        let target = channel.target();
        if let Some(node_name) = target.node().name() {
            if let Some(lst) = channels_per_joint.get_mut(node_name) {
                lst.push(channel);
            } else {
                channels_per_joint.insert(node_name, vec![(channel)]);
            }
        }
    }
    
    for index in 0..skeleton.joint_names.len() {
        let mut new_track = RawJointTrack::default();
        if let Some(name)  = skeleton.joint_names[index].as_ref() {
            if let Some(channels) = channels_per_joint.get(name.as_str()) {
                for channel in channels {
                    sample_animation_channel(data,&mut raw_animation.duration, channel,&mut new_track,30f32)?;
                }
            }
        }

       

        let rest_pos = &skeleton.joint_rest_poses[index];
        if new_track.translations.is_empty() {
            new_track.translations.push(RawTranslationKey { time: 0f32, value: rest_pos.position });           
        }
        if new_track.scales.is_empty() {
            new_track.scales.push(RawScaleKey { time: 0f32, value: rest_pos.scale });
        }
        if new_track.rotations.is_empty() {
            new_track.rotations.push(RawRotationKey { time: 0f32, value: rest_pos.rotation });
        }
        raw_animation.tracks.push(new_track);
    }
    let animation = AnimationBuilder::build(&raw_animation);
    Ok(animation)
}

fn sample_animation_channel(data:&ImportData,duration:&mut f32,channel:&Channel,track:&mut RawJointTrack,rate:f32) -> Result<()> {
    let sampler = channel.sampler();
    let input = sampler.input();
    let output = sampler.output();
    let max_value = input.max();
    let max_duration:f32 = max_value.as_ref()
                                    .and_then(|v| v.as_array())
                                    .map(|v| &v[0])
                                    .and_then(|v| v.as_f64()).unwrap_or(0f64) as f32;
    if max_duration > *duration {
        *duration = max_duration;
    }
    let buffer_view = input.view().ok_or(anyhow!("view nil"))?;
    let istride = buffer_view.stride().unwrap_or(0);
    
    let start = buffer_view.offset() + input.offset() as usize ;
    let end = start + (istride * input.count());
    let buffer = &data.1[buffer_view.buffer().index()][start..end];
    let timestamps:&[f32] =  unsafe { std::slice::from_raw_parts(buffer.as_ptr() as * const f32, input.count()) };
   
    let out_buffer_view = output.view().ok_or(anyhow!("view nil"))?;
    let ostride = out_buffer_view.stride().unwrap_or(0);
    let out_buffer_start:usize = out_buffer_view.offset() + output.offset()  as usize;
    let out_buffer_end:usize = out_buffer_start + (ostride * output.count());
    let out_buffer:&[u8] = &data.1[out_buffer_view.buffer().index()][out_buffer_start..out_buffer_end];

    match channel.target().property() {
        Property::Translation => {
            sample_channel::<RawTranslationKey,Vec3>(sampler.interpolation(),
                    out_buffer,output.count(),
                          &timestamps,rate,*duration,
                      &mut track.translations,RawTranslationKey::new);
        },
        Property::Scale => {
            
            sample_channel::<RawScaleKey,Vec3>(sampler.interpolation(),
                    out_buffer,output.count(),
                          &timestamps,rate,*duration,
                      &mut track.scales,RawScaleKey::new);
        },
        Property::Rotation => {
            sample_channel::<RawRotationKey,[f32;4]>(sampler.interpolation(),
                    out_buffer,output.count(),
                          &timestamps,rate,*duration,
                      &mut track.rotations,RawRotationKey::new);
            for key in track.rotations.iter_mut() {
                key.value = key.value.normalize();
            }
        },
         _ => {}
    }
    Ok(())
}


fn sample_channel<T,E:Clone>(interpolation:Interpolation,output:&[u8],
                          len:usize,timestamps:&[f32],
                          _rate:f32,_duration:f32,keys:&mut Vec<T>,f:fn(t:f32,v:E) -> T) where T:Debug {
    match interpolation {
        Interpolation::Linear => {
            sample_line_channel::<T,E>(output,len,timestamps,keys,f);
        },
        Interpolation::Step => {
            //sample_step_channel::<T,E>(data,output,len,timestamps,keys,f);
        },
        Interpolation::CubicSpline => {
            //sample_cubicspline_channel::<T,E>(data,output,len,timestamps,keys,f);
        },
    }
}

fn sample_line_channel<T,E:Clone>(output:&[u8],len:usize,timestamps:&[f32],keys:&mut Vec<T>,f:fn(t:f32,e:E) -> T) {
    if output.len() == 0 { keys.clear(); return; }
    let key_values:&[E] =  unsafe { std::slice::from_raw_parts(output.as_ptr() as * const E, len) };
    for index in 0..key_values.len() {
        keys.push(f(timestamps[index],key_values[index].clone()));
    }
}