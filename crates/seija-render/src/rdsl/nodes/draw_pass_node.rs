use bevy_ecs::prelude::{World, Entity};
use lite_clojure_eval::Variable;
use anyhow::{Result};
use seija_asset::{Handle, Assets};
use wgpu::{Operations, Color, CommandEncoder};
use crate::{IUpdateNode, RenderContext, query::QuerySystem, 
            resource::{Mesh, RenderResourceId, RenderResources}, 
            material::{Material, MaterialStorage}, pipeline::PipelineCache, rdsl::atom::Atom};

#[derive(Debug,PartialEq, Eq)] 
pub enum PassError {
    ErrArg,
    ErrInput(usize),
    ErrTargetView,
    ErrDepthView,
    MissMesh,
    MissMaterial,
    ErrUBOIndex,
    TextureNotReady
}


pub struct DrawPassNode {
    query_index:usize,
    camera_entity:Option<Entity>,
    textures:Vec<*mut Atom<RenderResourceId>>,
    depth:Option<*mut Atom<RenderResourceId>>,
    pass_name:String,

    operations:Operations<Color>,

}

impl Default for DrawPassNode {
    fn default() -> Self {
        DrawPassNode { 
            query_index: 0, 
            camera_entity: None, 
            textures: vec![], 
            depth: None,
            pass_name:String::default(),
            operations: wgpu::Operations {
                load:wgpu::LoadOp::Clear(Color {r:0f64,g:0f64,b:0f64,a:1f64 }),
                store:true  
            },
        }
    }
}

impl IUpdateNode for DrawPassNode {
    fn update_params(&mut self,params:Vec<Variable>) -> Result<()> {
       if let Some(index) = params[0].cast_int() {
          self.query_index = index as usize;
       }
       if let Some(index) = params[1].cast_int() {
          self.camera_entity = Some(Entity::from_bits(index as u64)); 
       }
       if let Some(list) = params[2].cast_vec() {
            for item in list.borrow().iter() {
                if let Some(u8_ptr) = item.cast_userdata() {
                    let ptr = u8_ptr as *mut Atom<RenderResourceId>;
                     self.textures.push(ptr);
                 }
            }
        }
        if let Some(u8_ptr) = params[3].cast_userdata() {
            let ptr = u8_ptr as *mut Atom<RenderResourceId>;
            self.depth = Some(ptr)
        }
        if let Some(pass_name) = params[4].cast_string() {
            self.pass_name = pass_name.borrow().clone();
        }

        Ok(())
    }

    fn update(&mut self,world:&mut World,ctx:&mut RenderContext) {
        
        let mut command = ctx.command_encoder.take().unwrap();
        match self.draw(world, ctx, &mut command) {
            Err(err) => {
                if err != PassError::TextureNotReady {
                    log::error!("draw pass error:{:?}",err);
                }
            },
            Ok(draw_count) => {
                if draw_count > 0 { ctx.frame_draw_pass += 1; }
            }
        }
        ctx.command_encoder = Some(command)
    }
}

impl DrawPassNode {
    pub fn draw(&self,world:&mut World,ctx:&mut RenderContext,command:&mut CommandEncoder) -> Result<u32,PassError> {
        let mut render_query = world.query::<(&Handle<Mesh>,&Handle<Material>)>();
        let meshs = world.get_resource::<Assets<Mesh>>().unwrap();
        let material_storages = world.get_resource::<MaterialStorage>().unwrap();
        let mats = material_storages.mateials.read();
        let query_system = world.get_resource::<QuerySystem>().unwrap();
        let pipeline_cahce = world.get_resource::<PipelineCache>().unwrap();
        let view_query = query_system.querys[self.query_index].read();
        
        let mut draw_count:u32 = 0;
       
        let mut render_pass = self.create_render_pass(&ctx.resources,  command)?;
       
        for entity in view_query.list.iter() {
            if let Ok((hmesh,hmat)) = render_query.get(world, *entity) { 
                let mesh = meshs.get(&hmesh.id).ok_or(PassError::MissMesh)?;
                let material = mats.get(&hmat.id).ok_or(PassError::MissMaterial)?;
                
                if !material.is_ready(&ctx.resources) { continue }
               
                if let Some(pipelines)  = pipeline_cahce.get_pipeline(&material.def.name, mesh) {
                    if let Some(mesh_buffer_id)  = ctx.resources.get_render_resource(&hmesh.id, 0) {
                        for pipeline in pipelines.pipelines.iter() {

                            if pipeline.tag != self.pass_name {
                                 //log::warn!("skip tag :{}",&pipeline.tag);
                                 continue; 
                            }
                            
                            let vert_buffer = ctx.resources.get_buffer_by_resid(&mesh_buffer_id).unwrap();
                           
                            let oset_index = pipeline.set_binds(self.camera_entity, entity, &mut render_pass, &ctx.ubo_ctx);
                            if oset_index.is_none() { continue }
                            let mut set_index = oset_index.unwrap();                
                            if material.props.def.infos.len() > 0 {   
                                render_pass.set_bind_group(set_index, material.bind_group.as_ref().unwrap(), &[]);
                                set_index += 1;
                            }
                            if material.texture_props.textures.len() > 0 {
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
        }
        Ok(draw_count)
    }

    fn create_render_pass<'a>(&self,res:&'a RenderResources,command:&'a mut CommandEncoder) -> Result<wgpu::RenderPass<'a>,PassError> {
        let mut color_attachments:Vec<wgpu::RenderPassColorAttachment> = vec![];
        for atom in self.textures.iter() {
            let atom_ref = unsafe { &**atom };
            if !res.is_ready(atom_ref.inner()) {
                return Err(PassError::TextureNotReady);
            }
            let texture = res.get_texture_view_by_resid(atom_ref.inner()).ok_or(PassError::ErrTargetView)?;
            color_attachments.push(wgpu::RenderPassColorAttachment {
                view:texture,
                resolve_target:None,
                ops:self.operations
            });
        }
        let mut depth_view:Option<wgpu::RenderPassDepthStencilAttachment> = None;
        if let Some(atom_depth) = self.depth {
            let atom_ref = unsafe { &*atom_depth };
            if !res.is_ready(atom_ref.inner()) {
                return Err(PassError::TextureNotReady);
            }
            let texture_view = res.get_texture_view_by_resid(atom_ref.inner()).ok_or(PassError::ErrDepthView)?;
            depth_view = Some(wgpu::RenderPassDepthStencilAttachment {
                view:texture_view,
                stencil_ops: None,
                depth_ops: Some(Operations {
                    load:  wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
            });
        }
        let pass_desc = wgpu::RenderPassDescriptor {
            label:None,
            color_attachments:color_attachments.as_slice(),
            depth_stencil_attachment:depth_view
        };
        let pass = command.begin_render_pass(&pass_desc);
        Ok(pass)
    }
}