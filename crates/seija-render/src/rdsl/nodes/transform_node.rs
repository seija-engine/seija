use bevy_ecs::prelude::{World, Entity, Added, With, Changed};
use lite_clojure_eval::Variable;
use anyhow::{Result,anyhow};
use seija_asset::Handle;
use seija_transform::Transform;
use crate::{rdsl::{node::IUpdateNode}, RenderContext, uniforms::backends::TransformBackend, UniformIndex, resource::Mesh, material::Material};
#[derive(Default)]
pub struct TransfromNode {
    ubo_name:String,

    backend:Option<TransformBackend>,
    name_index:Option<UniformIndex>,
}

impl IUpdateNode for TransfromNode {
    fn update_params(&mut self,params:Vec<Variable>) {
        if let Some(string) = params.get(0).and_then(Variable::cast_string) {
            self.ubo_name = string.borrow().clone();
        }
    }

    fn init(&mut self,_:& World,ctx:&mut RenderContext) -> Result<()> {
        let info = ctx.ubo_ctx.info.get_info(&self.ubo_name).ok_or(anyhow!("not found ubo {}",&self.ubo_name))?;
        let backend = TransformBackend::from_def(&info.props).map_err(|v| anyhow!("TransformBackend  err:{}",v.as_str()))?;
        self.backend = Some(backend);
        self.name_index = Some(ctx.ubo_ctx.get_index(self.ubo_name.as_str()).ok_or(anyhow!("not found {}",self.ubo_name.as_str()))?);
        Ok(())
    }

    fn prepare(&mut self,world:&mut World,ctx:&mut RenderContext) {
        let mut added_transform = world.query_filtered::<Entity,(Added<Transform>,With<Handle<Mesh>>,With<Handle<Material>>)>();
        if let Some(name_index) = self.name_index {
            for v in added_transform.iter(&world) {
                ctx.ubo_ctx.add_component(&name_index,v.id(),&mut ctx.resources)
            }
    
            for rm_e in world.removed::<Transform>() {
               ctx.ubo_ctx.remove_component(&name_index, rm_e.id());
            }
        }
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
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
    }
}