use bevy_ecs::{world::World, prelude::Entity, query::{Added, With, Changed}};
use lite_clojure_eval::Variable;
use lite_clojure_frp::FRPSystem;
use seija_asset::Handle;
use seija_transform::Transform;
use smol_str::SmolStr;
use anyhow::{Result,anyhow};

use crate::{RenderContext, dsl_frp::errors::Errors, uniforms::backends::TransformBackend, UniformIndex, resource::Mesh, material::Material};

use super::IUpdateNode;


pub struct TransfromNode {
    ubo_name:SmolStr,
    backend:Option<TransformBackend>,
    name_index:Option<UniformIndex>,
}

impl TransfromNode {
    pub fn from_args(args:Vec<Variable>) -> Result<Box<dyn IUpdateNode>> {
        let name = args.get(0).and_then(Variable::cast_string)
                              .ok_or(Errors::TypeCastError("string"))?;
        let br_names = name.borrow();
        Ok(Box::new(TransfromNode { ubo_name:br_names.clone().into(),backend:None,name_index:None }))
    }

    fn prepare(&mut self,world:&mut World,ctx:&mut RenderContext) -> Result<()> {
        let mut added_transform = world.query_filtered::<Entity,(Added<Transform>,With<Handle<Mesh>>,With<Handle<Material>>)>();
        if let Some(name_index) = self.name_index {
            for v in added_transform.iter(&world) {
                ctx.ubo_ctx.add_component(&name_index,v.id(),&mut ctx.resources)
            }
    
            for rm_e in world.removed::<Transform>() {
               ctx.ubo_ctx.remove_component(&name_index, rm_e.id());
            }
        }
        Ok(())
    }
}

impl IUpdateNode for TransfromNode {
    fn init(&mut self,_:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> anyhow::Result<()> {
        let info = ctx.ubo_ctx.info.get_info(&self.ubo_name).ok_or(Errors::NotFoundUBO(self.ubo_name.clone()))?;
        let backend = TransformBackend::from_def(&info.props).map_err(|v| anyhow!("transform backend err:{}",v.as_str()))?;
        self.backend = Some(backend);
        Ok(())
    }

    fn active(&mut self,_world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        let name_index = ctx.ubo_ctx.get_index(self.ubo_name.as_str()).ok_or(anyhow!("err ubo name"))?;
        self.name_index = Some(name_index);
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext,frp_sys:&mut FRPSystem) -> Result<()> {
        self.prepare(world, ctx)?;
        let mut trans = world.query_filtered::<(Entity,&Transform),(Changed<Transform>,With<Handle<Mesh>>,With<Handle<Material>>)>();
        for (e,t) in trans.iter(world) { 
            if let Some(key) = self.name_index {
                if let Some(backend) = self.backend.as_ref() {
                    ctx.ubo_ctx.set_buffer(&key, Some(e.id()), |buffer| {
                        backend.set_transform(&mut buffer.buffer,  &t.global().matrix());
                    });  
                }
            }
        }
        Ok(())
    }
}