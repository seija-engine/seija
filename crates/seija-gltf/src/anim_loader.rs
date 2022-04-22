use std::{collections::{ HashMap}};

use gltf::{Skin, Node, Document, animation::{Channel, Sampler, Property, Interpolation}, Accessor};
use seija_skeleton3d::{offine::{raw_skeleton::{RawSkeleton, RawJoint}, skeleton_builder::SkeletonBuilder, raw_animation::{RawAnimation, RawJointTrack}}, Skeleton};
use seija_transform::{Transform, TransformMatrix};

use crate::{ImportData, GltfError};

pub fn load_skeleton(gltf:&ImportData) -> Result<Option<RawSkeleton>,GltfError> {
    
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

    let mut skeleton:RawSkeleton = RawSkeleton::default();
    for root_node in roots {
        let joint =import_node_to_joint(&root_node);
        skeleton.roots.push(joint);
    }
    Ok(Some(skeleton))
}

pub fn load_animations(data:&ImportData,skeleton:&Skeleton) {
    for anim in data.0.animations() {
        import_animation(data,&anim, skeleton);
    }
}


fn import_animation(data:&ImportData,animation:&gltf::Animation,skeleton:&Skeleton) {
    let mut raw_animation = RawAnimation::default();
    raw_animation.name = animation.name().unwrap_or("none").to_string();
    raw_animation.duration = 0f32;
    let mut channels_per_joint:HashMap<&str,(Node,Vec<Channel>)> = HashMap::default();
    for channel in animation.channels() {
        let target = channel.target();
        if let Some(node_name) = target.node().name() {
            if let Some(lst) = channels_per_joint.get_mut(node_name) {
                lst.1.push(channel);
            } else {
                channels_per_joint.insert(node_name, (target.node().clone(),vec![(channel)]));
            }
        }
    }
    
    for index in 0..skeleton.joint_names.len() {
        let mut new_track = RawJointTrack::default();
        if let Some(name)  = skeleton.joint_names[index].as_ref() {
            if let Some((node,channels)) = channels_per_joint.get(name.as_str()) {
                for channel in channels {
                    sample_animation_channel(data,&mut raw_animation.duration, channel,&mut new_track,30f32);
                }
            }
            
        }
    }
}

fn sample_animation_channel(data:&ImportData,duration:&mut f32,channel:&Channel,track:&mut RawJointTrack,rate:f32) {
    let sampler = channel.sampler();
    
    let input = sampler.input();
    
    let max_value = input.max();
    let max_duration:f32 = max_value.as_ref().and_then(|v| v.as_array()).map(|v| &v[0]).and_then(|v| v.as_f64()).unwrap_or(0f64) as f32;
    if max_duration > *duration {
        *duration = max_duration;
    }
    if let Some(buffer_view) = input.view() {
        let start = buffer_view.offset() as usize;
        let end = (buffer_view.offset() + buffer_view.length()) as usize;
        let buffer = &data.1[buffer_view.buffer().index()][start..end];
        let timestamps:&[f32] =  unsafe { std::slice::from_raw_parts(buffer.as_ptr() as * const f32, buffer_view.length()) };
        match channel.target().property() {
            Property::Translation => {
               sample_channel(data, sampler.interpolation(), &sampler.output(),&timestamps,rate,*duration);
            },
            Property::Scale => {},
            Property::Rotation => {},
            _ => {}
        }   
    }
}

fn sample_channel(data:&ImportData,interpolation:Interpolation,output:&Accessor,timestamps:&[f32],rate:f32,duration:f32) {
    match interpolation {
        Interpolation::Linear => {

        },
        Interpolation::Step => {
            
        },
        Interpolation::CubicSpline => {

        },
    }
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

fn find_skin_root_joint<'a>(skins:&Vec<Skin<'a>>,doc:&'a Document) -> Vec<Node<'a>> {
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

#[test]
fn test_load() {

    let import_data:ImportData = gltf::import("res/Fox.gltf").unwrap();
    let raw_skeleton = load_skeleton(&import_data).unwrap().unwrap();
    let out_string = format!("{:?}",&raw_skeleton);
    let skeleton = SkeletonBuilder::build(&raw_skeleton);
    load_animations(&import_data,&skeleton);
   
}