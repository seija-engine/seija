use bevy_ecs::prelude::{World, Entity, Added, With};
use seija_asset::{Handle, Assets};
use seija_render::{graph::INode, RenderContext, resource::{RenderResourceId, Mesh}, material::Material, UniformIndex};

use crate::{RuntimeSkeleton, Skin};

pub struct SkeletonNode {
    ubo_name:String,
    name_index:Option<UniformIndex>,
    joints_index:Option<usize>
}

impl SkeletonNode {
    pub fn new(name:String) -> Self {
        SkeletonNode {ubo_name:name, name_index:None,joints_index:None }
    }
}

impl INode for SkeletonNode {
    fn init(&mut self, _world: &mut World, ctx:&mut RenderContext) {
        self.name_index = ctx.ubo_ctx.get_index(self.ubo_name.as_str());
        if let Some(info) = ctx.ubo_ctx.info.get_info(&self.ubo_name) {
            if let Some(idx) = info.props.get_offset("jointMats", 0) {
                self.joints_index = Some(idx);
            } else {
                self.joints_index = None;
                log::error!("not found jointMats in {}",self.ubo_name);
            }
        }
    }

    fn prepare(&mut self, world: &mut World,ctx:&mut RenderContext) {
        let mut added_skins = world.query_filtered::<Entity,
                                                                                    (Added<Handle<RuntimeSkeleton>>,
                                                                                     With<Handle<Mesh>>,
                                                                                     With<Handle<Material>>)>();
        if let Some(name_index) = self.name_index.as_ref() {
            for v in added_skins.iter(&world) {
                ctx.ubo_ctx.add_component(name_index, v.id(), &mut ctx.resources);
            }
    
            for rm_e in world.removed::<Handle<RuntimeSkeleton>>() {
                ctx.ubo_ctx.remove_component(name_index, rm_e.id());
            }
        }
        
    }

    fn update(&mut self,world: &mut World,ctx:&mut RenderContext,_:&Vec<Option<RenderResourceId>>,_:&mut Vec<Option<RenderResourceId>>) {
        let mut rt_skins = world.query_filtered::<(Entity,&Handle<RuntimeSkeleton>,&Handle<Skin>),
                                                                                   (With<Handle<Mesh>>,With<Handle<Material>>)>();
        let rt_skeletons = world.get_resource::<Assets<RuntimeSkeleton>>().unwrap();
        let skins = world.get_resource::<Assets<Skin>>().unwrap();
        let joint_index = self.joints_index.unwrap();
        let name_index = self.name_index.unwrap();
        for (e,runtime_skeleton,h_skin) in rt_skins.iter(world) {
            let rt_skeleton = rt_skeletons.get(&runtime_skeleton.id).unwrap();
            let skin = skins.get(&h_skin.id).unwrap();
            let len = rt_skeleton.mat4s.len();
            let mut out_f32s:Vec<f32> = Vec::with_capacity(len * 16);
            for index in 0..rt_skeleton.mat4s.len() {
                let world_mat = &rt_skeleton.mat4s[index];
                let inv_mat = &skin.mats()[index];
                let mul_mat = world_mat.mul_mat4(inv_mat);
                
                out_f32s.extend_from_slice(&mul_mat.to_cols_array());
            }

            ctx.ubo_ctx.set_buffer(&name_index, Some(e.id()), |buffer| {
                let u8_ptr =  unsafe { core::slice::from_raw_parts(out_f32s.as_ptr() as *const u8, out_f32s.len() *4) };
                buffer.buffer.write_bytes_(joint_index, u8_ptr)
            });
        }
    }
}