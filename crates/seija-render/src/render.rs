use bevy_ecs::prelude::{World};
use seija_asset::{AssetEvent};
use bevy_ecs::event::{Events, ManualEventReader};
use seija_core::window::AppWindow;
use seija_winit::event::{WindowCreated, WindowResized};
use std::{borrow::Cow, sync::Arc};
use wgpu::{CommandEncoderDescriptor, Device, Instance, Queue};
use crate::rdsl::RenderMain;
use crate::render_context::RenderContext;
use crate::resource::{self, Mesh, RenderResources, Texture};
pub struct AppRender {
    pub instance: wgpu::Instance,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,

    pub main:RenderMain,

    pub window_resized_event_reader: ManualEventReader<WindowResized>,
    pub window_created_event_reader: ManualEventReader<WindowCreated>,
    mesh_event_reader:ManualEventReader<AssetEvent<Mesh>>,
    texture_event_reader:ManualEventReader<AssetEvent<Texture>>,
}

pub struct Config {
    pub device_label: Option<Cow<'static, str>>,
    pub backed: wgpu::BackendBit,
    pub power_pref: wgpu::PowerPreference,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
}

impl Default for Config {
    fn default() -> Self {
        let mut limits:wgpu::Limits = Default::default();
        limits.max_bind_groups = 8;
        Self {
            device_label: None,
            backed: wgpu::BackendBit::VULKAN,
            power_pref: wgpu::PowerPreference::HighPerformance,
            features: Default::default(),
            limits,
        }
    }
}

impl AppRender {
    pub fn new_sync(config: Config) -> AppRender {
        futures_lite::future::block_on(Self::new(config))
    }

    pub async fn new(config: Config) -> AppRender {
        let (device,instance,queue) = AppRender::create_wgpu(config).await;
        AppRender {
            instance,
            device,
            queue,
            window_created_event_reader:Default::default(),
            window_resized_event_reader:Default::default(),
            mesh_event_reader:Default::default(),
            texture_event_reader:Default::default(),
            main:RenderMain::new()
        }
    }

    async fn create_wgpu(config:Config) -> (Arc<Device>,Instance,Queue) {
        let instance = wgpu::Instance::new(config.backed);
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: config.power_pref,
                compatible_surface: None,
            })
            .await
            .expect("Unable to find a GPU!");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: config.device_label.as_ref().map(|a| a.as_ref()),
                    features: config.features,
                    limits: config.limits,
                },
                None,
            )
            .await
            .unwrap();
        (Arc::new(device),instance,queue)
    }


    pub fn update(&mut self, world: &mut World,ctx:&mut RenderContext) {
        ctx.frame_draw_pass = 0;
        ctx.command_encoder = Some(self.device.create_command_encoder(&CommandEncoderDescriptor::default()));
        self.update_winodw_surface(world,&mut ctx.resources);
        let _ = ctx.resources.next_swap_chain_texture();
        ctx.ubo_ctx.update(&mut ctx.resources,ctx.command_encoder.as_mut().unwrap());
       
        ctx.mat_system.update(world, &mut ctx.resources, ctx.command_encoder.as_mut().unwrap());
       
        
        resource::update_mesh_system(world,&mut self.mesh_event_reader,ctx);
       
        resource::update_texture_system(world, &mut self.texture_event_reader, ctx);
        self.main.update(ctx,world);
        
        let command_buffer = ctx.command_encoder.take().unwrap().finish();
        self.queue.submit(Some(command_buffer));
        if ctx.frame_draw_pass > 0 {
            ctx.resources.clear_swap_chain_texture();
            ctx.frame_draw_pass = 0;
        }
       
    }

    fn update_winodw_surface(&mut self, world: &mut World,render_res:&mut RenderResources) {
        let mut is_create_window = false;
        {
            let created_events = world.get_resource::<Events<WindowCreated>>().unwrap();
            for _ in self.window_created_event_reader.iter(&created_events) {
                is_create_window = true;
            }
        };
        if is_create_window {
            let app_window = world.get_resource::<AppWindow>().unwrap();   
            let surface = unsafe { self.instance.create_surface(app_window) };
            render_res.set_main_surface(surface);
            render_res.create_swap_chain(app_window.width(), app_window.height(), app_window.vsync());
        }

        let mut resize:Option<WindowResized> = None;
        {
            let resize_events = world.get_resource_mut::<Events<WindowResized>>().unwrap();
            for resize_event in self.window_resized_event_reader.iter(&resize_events) {
                resize = Some(*resize_event);
            }
        };
        if let Some(_) = resize {
            let app_window = world.get_resource::<AppWindow>().unwrap();   
            if app_window.width() > 0 && app_window.height() > 0 {
                render_res.clear_swap_chain_texture();
                render_res.create_swap_chain(app_window.width(), app_window.height(), app_window.vsync());
            }
        }
        
    }
}
