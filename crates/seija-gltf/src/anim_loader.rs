use std::{collections::HashMap, fmt::Debug};

use glam::{Vec3, Mat4};
use gltf::{Node, Document, animation::{Channel, Property, Interpolation},};
use seija_skeleton3d::{offine::{raw_skeleton::{RawSkeleton, RawJoint}, skeleton_builder::SkeletonBuilder, raw_animation::{RawAnimation, RawJointTrack, RawTranslationKey, RawScaleKey, RawRotationKey}, animation_builder::AnimationBuilder}, Skeleton, AnimationSet, Animation, Skin};
use seija_transform::TransformMatrix;

use crate::{ImportData, GltfError};

pub fn load_skeleton(gltf:&ImportData) -> Result<Option<Skeleton>,GltfError> {
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

pub fn load_animations(data:&ImportData,skeleton:&Skeleton) -> Result<AnimationSet,GltfError> {
    let mut anim_set = AnimationSet::default();
    for gltf_anim in data.0.animations() {
       let animation = import_animation(data,&gltf_anim, skeleton)?;
       anim_set.add(animation);
    }
    Ok(anim_set)
}


fn import_animation(data:&ImportData,animation:&gltf::Animation,skeleton:&Skeleton) -> Result<Animation,GltfError> {
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

fn sample_animation_channel(data:&ImportData,duration:&mut f32,channel:&Channel,track:&mut RawJointTrack,rate:f32) -> Result<(),GltfError> {
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
    let buffer_view = input.view().ok_or(GltfError::LoadAnimError)?;
    let istride = buffer_view.stride().unwrap_or(0);
    
    let start = buffer_view.offset() + input.offset() as usize ;
    let end = start + (istride * input.count());
    let buffer = &data.1[buffer_view.buffer().index()][start..end];
    let timestamps:&[f32] =  unsafe { std::slice::from_raw_parts(buffer.as_ptr() as * const f32, input.count()) };
   
    let out_buffer_view = output.view().ok_or(GltfError::LoadImageError)?;
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
/* 
fn sample_step_channel<T,E:Clone>(data:&ImportData,output:&[u8],len:usize,timestamps:&[f32],keys:&mut Vec<T>,f:fn(t:f32,e:E) -> T) {
    todo!()
}

fn sample_cubicspline_channel<T,E:Clone>(data:&ImportData,output:&[u8],len:usize,timestamps:&[f32],keys:&mut Vec<T>,f:fn(t:f32,e:E) -> T) {
    todo!()
}*/

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
