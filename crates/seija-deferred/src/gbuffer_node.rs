use bevy_ecs::prelude::{World, Entity};
use seija_asset::Handle;
use seija_core::{event::{ManualEventReader, Events}, window::AppWindow};
use seija_render::{graph::INode, RenderContext, resource::{RenderResourceId, RenderResources, Mesh}, wgpu::{self, Operations}, camera::camera::Camera, material::{Material, MaterialStorage, RenderPath}};
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

        if let Err(err) = self.draw(world,ctx, inputs) {
            log::error!("{:?}",err);
        }
    }
}

impl GBufferNode {
    pub fn new() -> Self {
        let mut gbuffer = GBufferNode::default();
        gbuffer.texture_formats.push(wgpu::TextureFormat::Rgba32Float); //position
        gbuffer.texture_formats.push(wgpu::TextureFormat::Rgba8Unorm); //basecolor
        gbuffer.texture_formats.push(wgpu::TextureFormat::Rgba8Unorm); //normal
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

    fn draw(&self,world:&mut World,ctx:&mut RenderContext,inputs:&Vec<Option<RenderResourceId>>) -> Result<(),GBfferError> {
        let mut command = ctx.command_encoder.take().unwrap();
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

        let mut render_pass = command.begin_render_pass(&wgpu::RenderPassDescriptor {
            label:None,
            color_attachments:&color_attachs,
            depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {
                view:depth_view,
                stencil_ops: None,
                depth_ops: Some(Operations { load: wgpu::LoadOp::Clear(1.0), store: true, }),
            }),
        });

        let mut camera_query = world.query::<(Entity,&Transform,&Camera)>();
        let mut render_query = world.query::<(Entity,&Handle<Mesh>,&Handle<Material>)>();
        let mat_storages = world.get_resource::<MaterialStorage>().unwrap();
        let materials = mat_storages.mateials.read();

        for (camera_e,_,camera) in camera_query.iter(world) {
            for ves in camera.view_list.values.iter() {
                for view_entity in ves.value.iter() {
                    if let Ok((_,hmesh,hmat))  = render_query.get(world, view_entity.entity) {
                        let material = materials.get(&hmat.id).ok_or(GBfferError::MissMaterial)?;
                        if material.def.path != RenderPath::Deferred { continue; }
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
    MissMaterial
}