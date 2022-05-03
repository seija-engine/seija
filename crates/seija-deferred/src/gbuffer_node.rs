use bevy_ecs::prelude::{World, Entity};
use seija_asset::{Handle, Assets};
use seija_core::{event::{ManualEventReader, Events}, window::AppWindow};
use seija_render::{graph::INode, RenderContext, resource::{RenderResourceId, RenderResources, Mesh}, wgpu::{self, Operations}, camera::camera::Camera, material::{Material, MaterialStorage, RenderPath}, pipeline::PipelineCache};
use seija_transform::Transform;
use seija_winit::event::{WindowCreated, WindowResized};

#[derive(Default)]
pub struct GBufferNode {
    texture_formats:Vec<wgpu::TextureFormat>,
    textures:Vec<Option<RenderResourceId>>,

    pub resized_event_reader: ManualEventReader<WindowResized>,
    pub created_event_reader: ManualEventReader<WindowCreated>,
}

impl INode for GBufferNode {
    fn input_count(&self) -> usize { 1 }
    fn output_count(&self) -> usize { self.texture_formats.len() }

    fn prepare(&mut self, world: &mut World, ctx:&mut RenderContext) {
        
        self.prepare_textures(world,&mut ctx.resources);
    }

    fn update(&mut self,world: &mut World,
              ctx:&mut RenderContext,
              inputs:&Vec<Option<RenderResourceId>>,
              outputs:&mut Vec<Option<RenderResourceId>>) {
        *outputs = self.textures.clone();
        let mut command:wgpu::CommandEncoder = ctx.command_encoder.take().unwrap();
        if let Err(err) = self.draw(world,ctx, inputs,&mut command) {
            log::error!("{:?}",err);
        }
        ctx.command_encoder = Some(command)
    }
}

impl GBufferNode {
    pub fn new() -> Self {
        let mut gbuffer = GBufferNode::default();
        gbuffer.texture_formats.push(wgpu::TextureFormat::Rgba32Float); //position
        gbuffer.texture_formats.push(wgpu::TextureFormat::Rgba32Float); //basecolor
        //gbuffer.texture_formats.push(wgpu::TextureFormat::Bgra8UnormSrgb); //normal
        gbuffer
    }

    fn prepare_textures(&mut self,world:&mut World,res:&mut RenderResources) {
        let mut make_texture = false;
        if self.textures.len() != self.texture_formats.len() { make_texture = true }
        if let Some(events) =  world.get_resource::<Events<WindowCreated>>() {
            make_texture =  self.created_event_reader.iter(events).count() > 0;
        }
        if let Some(events) =  world.get_resource::<Events<WindowResized>>() {
            for ev_resize in self.resized_event_reader.iter(events) {
                if ev_resize.width > 0f32 && ev_resize.height > 0f32 { make_texture = true; }
            }
        }

        if make_texture {
            for old_texture in self.textures.iter() {
                if let Some(old_texture) = old_texture {
                    res.remove_texture(old_texture);
                }
            }
            let app_window = world.get_resource::<AppWindow>().unwrap();
            self.textures = self.make_textures(app_window.width(), app_window.height(), res)
        }
    }

    fn draw(&self,world:&mut World,ctx:&mut RenderContext,inputs:&Vec<Option<RenderResourceId>>,command:&mut wgpu::CommandEncoder) -> Result<(),GBfferError> {
       
        let depth_view = inputs[0].as_ref()
                                                     .and_then(|id| ctx.resources.get_texture_view_by_resid(id))
                                                     .ok_or(GBfferError::ErrInput(0))?;
        let mut color_attachs:Vec<wgpu::RenderPassColorAttachment> = vec![];
        for res_id in self.textures.iter() {
            if let Some(view) = res_id.as_ref().and_then(|v| ctx.resources.get_texture_view_by_resid(&v)) {
                let color_attach = wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target:None,
                    ops:Default::default()
                };
                color_attachs.push(color_attach);
            }
        }

        let mut camera_query = world.query::<(Entity,&Transform,&Camera)>();
        let mut render_query = world.query::<(Entity,&Handle<Mesh>,&Handle<Material>)>();

        let mat_storages = world.get_resource::<MaterialStorage>().unwrap();
        let materials = mat_storages.mateials.read();

        let mut render_pass = command.begin_render_pass(&wgpu::RenderPassDescriptor {
            label:None,
            color_attachments:&color_attachs,
            depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {
                view:depth_view,
                stencil_ops: None,
                depth_ops: Some(Operations { load: wgpu::LoadOp::Clear(1.0), store: true, }),
            }),
        });

        let pipeline_cahce = world.get_resource::<PipelineCache>().unwrap();
        let meshs = world.get_resource::<Assets<Mesh>>().unwrap();

        for (camera_e,_,camera) in camera_query.iter(world) {
            for ves in camera.view_list.values.iter() {
                for view_entity in ves.value.iter() {
                    if let Ok((ve,hmesh,hmat))  = render_query.get(world, view_entity.entity) {
                        let material = materials.get(&hmat.id).ok_or(GBfferError::MissMaterial)?;
                        if material.def.path != RenderPath::Deferred || !material.is_ready(& ctx.resources) { continue; }
                        
                        let mesh = meshs.get(&hmesh.id).ok_or(GBfferError::MissMesh)?;
                        let fst_pipeline = pipeline_cahce.get_pipeline(&material.def.name, mesh)
                                                                          .and_then(|pipes| pipes.pipelines.get(0))
                                                                          .ok_or(GBfferError::MissPipeline)?;
                        if let Some(mesh_buffer_id)  = ctx.resources.get_render_resource(&hmesh.id, 0) {
                            let vert_buffer = ctx.resources.get_buffer_by_resid(&mesh_buffer_id).unwrap();
                            let mut set_index = fst_pipeline.set_binds(camera_e,&ve, &mut render_pass, &ctx.ubo_ctx)
                                                                               .ok_or(GBfferError::ErrUBOIndex)?;
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
                                render_pass.set_pipeline(&fst_pipeline.pipeline);
                
                                render_pass.draw_indexed(mesh.indices_range().unwrap(),0, 0..1);
                            } else {
                                render_pass.set_pipeline(&fst_pipeline.pipeline);
                                render_pass.draw(0..mesh.count_vertices() as u32, 0..1);
                            }
                        }

                    }
                }
            }
        }

    
        Ok(())
    }

    fn make_textures(&self,w:u32,h:u32,res:&mut RenderResources) -> Vec<Option<RenderResourceId>> {
        let mut textures = vec![];
        for format in self.texture_formats.iter() {
            let mut desc = wgpu::TextureDescriptor { 
                label: None,
                size: wgpu::Extent3d::default(),
                mip_level_count: 1,
                sample_count: 1, 
                dimension: wgpu::TextureDimension::D2, 
                format: format.clone(), 
                usage: wgpu::TextureUsage::RENDER_ATTACHMENT 
            };
            desc.size.width = w;
            desc.size.height = h;
            let texture_id = res.create_texture(&desc, &wgpu::TextureViewDescriptor::default());
            textures.push(Some(RenderResourceId::Texture(texture_id)));
        }
        textures
    }
}

#[derive(Debug)] 
enum GBfferError {
    ErrInput(usize),
    MissMaterial,
    MissMesh,
    MissPipeline,
    ErrUBOIndex
}