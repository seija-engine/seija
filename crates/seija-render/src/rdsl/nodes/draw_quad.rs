use bevy_ecs::{world::World, prelude::Entity};
use lite_clojure_eval::Variable;
use anyhow::{Result,anyhow, Context};
use seija_asset::{Assets, Handle, AssetServer};
use seija_transform::Transform;
use smol_str::SmolStr;
use wgpu::{Operations, Color, CommandEncoder};

use crate::{IUpdateNode, rdsl::atom::Atom, resource::{RenderResourceId, shape::Quad, Mesh}, material::{Material, MaterialDefineAsset}, RenderContext, pipeline::PipelineCache};

use super::{CommonError, create_render_pass};
#[derive(Default)]
pub struct DrawQuadNode {
    material_name:SmolStr,
    textures:Vec<*mut Atom<RenderResourceId>>,
    depth:Option<*mut Atom<RenderResourceId>>,
    mat_textures:Vec<*mut Atom<RenderResourceId>>,
    operations:Operations<Color>,
    pass_index:i32,
    quad_id:Option<Entity>
}

impl IUpdateNode for DrawQuadNode {
    fn update_params(&mut self,params:Vec<Variable>) -> Result<()> {
        let param_0 = params.get(0).and_then(Variable::cast_string).ok_or(anyhow!("0"))?;
        self.material_name = param_0.borrow().as_str().into();

        let param_1 = params.get(1).and_then(Variable::cast_vec).ok_or(anyhow!("1"))?;
        for item in param_1.borrow().iter() {
            if let Some(u8_ptr) = item.cast_userdata() {
                let ptr = u8_ptr as *mut Atom<RenderResourceId>;
                 self.textures.push(ptr);
             }
        }

        let param_2 = params.get(2).and_then(Variable::cast_userdata).ok_or(anyhow!("2"))?;
        self.depth = Some(param_2 as *mut Atom<RenderResourceId>);

        let param_3 = params.get(3).and_then(Variable::cast_vec).ok_or(anyhow!("3"))?;
        for item in param_3.borrow().iter() {
            if let Some(u8_ptr) = item.cast_userdata() {
                let ptr = u8_ptr as *mut Atom<RenderResourceId>;
                 self.mat_textures.push(ptr);
             }
        }

        if let Some(pass_index) = params.get(4).and_then(Variable::cast_int) {
            self.pass_index = pass_index as i32;
        } else {
            self.pass_index = -1;
        }
        Ok(())
    }

    fn init(&mut self,world:&mut World,_:&mut crate::RenderContext) -> anyhow::Result<()> {
        self.operations = wgpu::Operations {
            load:wgpu::LoadOp::Clear(Color {r:0f64,g:0f64,b:0f64,a:1f64 }),
            store:true  
        };
        let hmat:Handle<Material> =  {
            let server = world.get_resource::<AssetServer>().context(0)?;
            let h_def = server.get_asset_handle(&self.material_name)
                                                      .context(1)?
                                                      .typed::<MaterialDefineAsset>();
            let defs = world.get_resource::<Assets<MaterialDefineAsset>>().context(2)?;
            let define = defs.get(&h_def.id).context(4)?.define.clone();
            let mut material = Material::from_def(define, server).context(5)?;
            for (index,tex) in self.mat_textures.iter().enumerate() {
                let atom_ref = unsafe { &**tex };
                if let RenderResourceId::Texture(texture) = atom_ref.inner() {
                    material.texture_props.set(&format!("texture{}",index) , texture.clone_weak());
                }
            };
            world.get_resource_mut::<Assets<Material>>().context(6)?.add(material)
        };
        let hmesh = {
            let mut meshs = world.get_resource_mut::<Assets<Mesh>>().ok_or(anyhow!("0"))?;
            meshs.add(Quad::new(2f32).into())
        };
        let t = Transform::default();
        let mut commands = world.spawn();
        commands.insert(t);
        commands.insert(hmesh);
        commands.insert(hmat);
        self.quad_id = Some(commands.id());
        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut crate::RenderContext) {
        let mut command = ctx.command_encoder.take().unwrap();
        match self.draw(world, ctx, &mut command) {
            Err(err) => {
                if err != CommonError::TextureNotReady {
                    log::error!("draw pass error:{:?}",err);
                }
            },
            Ok(draw_count) => { if draw_count > 0 { ctx.frame_draw_pass += 1; } }
        }
        ctx.command_encoder = Some(command)
    }
}

impl DrawQuadNode {
    pub fn draw(&self,world:&mut World,ctx:&mut RenderContext,command:&mut CommandEncoder) -> Result<u32,CommonError> {
        let mut draw_count:u32 = 0;
        let meshs = world.get_resource::<Assets<Mesh>>().unwrap();
        let materials = world.get_resource::<Assets<Material>>().unwrap();
        let pipeline_cahce = world.get_resource::<PipelineCache>().unwrap();
        if let Some(quad_entity) = self.quad_id {
            let quad_ref = world.entity(quad_entity);
            let hmesh = quad_ref.get::<Handle<Mesh>>().ok_or(CommonError::MissMesh)?;
            let hmat = quad_ref.get::<Handle<Material>>().ok_or(CommonError::MissMaterial)?;
            let mesh = meshs.get(&hmesh.id).ok_or(CommonError::MissMesh)?;
            let material = materials.get(&hmat.id).ok_or(CommonError::MissMaterial)?;

            if !material.is_ready(&ctx.resources) { return Ok(0); }
            
            let mut render_pass = create_render_pass(&self.textures, self.operations, self.depth, &ctx.resources, command)?;
            if let Some(pipelines)  = pipeline_cahce.get_pipeline(&material.def.name, mesh) {
              
                if let Some(mesh_buffer_id)  = ctx.resources.get_render_resource(&hmesh.id, 0) {
                    let mut cur_pass_index = 0;
                    
                    for pipeline in pipelines.pipelines.iter() {
                        if self.pass_index != -1 && self.pass_index != cur_pass_index {
                            cur_pass_index += 1;
                            continue;
                        }
                        cur_pass_index += 1;
                        let vert_buffer = ctx.resources.get_buffer_by_resid(&mesh_buffer_id).unwrap();
                        let oset_index = pipeline.set_binds(None, &quad_entity, &mut render_pass, &ctx.ubo_ctx);
                        if oset_index.is_none() { continue }
                        if material.bind_group.is_none() || material.texture_props.bind_group.is_none() { return Ok(0u32); }
                        let mut set_index = oset_index.unwrap();
                        if material.props.def.infos.len() > 0 && material.bind_group.is_some() {   
                            render_pass.set_bind_group(set_index, material.bind_group.as_ref().unwrap(), &[]);
                            set_index += 1;
                        }
                        if material.texture_props.textures.len() > 0 && material.texture_props.bind_group.is_some() {
                            render_pass.set_bind_group(set_index, material.texture_props.bind_group.as_ref().unwrap(), &[]);
                        }
                        render_pass.set_vertex_buffer(0, vert_buffer.slice(0..));
                        if let Some(idx_id) = ctx.resources.get_render_resource(&hmesh.id, 1) {
                            let idx_buffer = ctx.resources.get_buffer_by_resid(&idx_id).unwrap();
                            render_pass.set_index_buffer(idx_buffer.slice(0..), mesh.index_format().unwrap());
                            render_pass.set_pipeline(&pipeline.pipeline);
                            render_pass.draw_indexed(mesh.indices_range().unwrap(),0, 0..1);
                        } else {
                            render_pass.set_pipeline(&pipeline.pipeline);
                            render_pass.draw(0..mesh.count_vertices() as u32, 0..1);
                        }
                        draw_count += 1;
                    }
                }
            }
        }
        Ok(draw_count)
    }
}