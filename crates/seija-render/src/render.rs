use bevy_ecs::prelude::{World,Res};
use seija_asset::{AssetEvent, Assets};
use seija_core::event::{EventReader, Events, ManualEventReader};
use seija_core::window::AppWindow;
use seija_winit::event::{WindowCreated, WindowResized};
use std::{borrow::Cow, sync::Arc};
use wgpu::{CommandEncoder, CommandEncoderDescriptor};
use crate::camera::camera::update_camera;
use crate::graph::{LinearGraphIter, RenderGraph};
use crate::material::{MaterialSystem};
use crate::resource::{self, Mesh, RenderResources};

#[derive(Default)]
pub struct RenderGraphContext {
    pub graph: RenderGraph,
    pub graph_iter: Arc<LinearGraphIter>
}


impl RenderGraphContext {
    pub fn build_iter(&mut self) {
        self.graph_iter = Arc::new(LinearGraphIter::from_graph(&self.graph));
    }
}

unsafe impl Send for RenderContext {}
unsafe impl Sync for RenderContext {}
pub struct RenderContext {
    pub resources:RenderResources,
    pub command_encoder:Option<CommandEncoder>,
}

pub struct AppRender {
    pub instance: wgpu::Instance,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,

    pub window_resized_event_reader: ManualEventReader<WindowResized>,
    pub window_created_event_reader: ManualEventReader<WindowCreated>,
    mesh_event_reader:ManualEventReader<AssetEvent<Mesh>>,
    material_sys:MaterialSystem,
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
        Self {
            device_label: None,
            backed: wgpu::BackendBit::VULKAN,
            power_pref: wgpu::PowerPreference::HighPerformance,
            features: Default::default(),
            limits: Default::default(),
        }
    }
}

impl AppRender {
    pub fn new_sync(config: Config) -> AppRender {
        futures_lite::future::block_on(Self::new(config))
    }

    pub async fn new(config: Config) -> AppRender {
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
        let arc_device = Arc::new(device);
        AppRender {
            instance,
            device: arc_device.clone(),
            queue,
            window_created_event_reader:Default::default(),
            window_resized_event_reader:Default::default(),
            mesh_event_reader:Default::default(),
            material_sys:MaterialSystem::default(),
        }
    }

    pub fn update(&mut self, world: &mut World, graph_ctx: &mut RenderGraphContext,render_ctx:&mut RenderContext) {
        render_ctx.command_encoder = Some(self.device.create_command_encoder(&CommandEncoderDescriptor::default()));
        self.update_winodw_surface(world,&mut render_ctx.resources);
        update_camera(world,render_ctx);
        render_ctx.resources.next_swap_chain_texture();
        self.material_sys.update(world,&self.device,render_ctx.command_encoder.as_mut().unwrap());
        graph_ctx.graph.prepare(world);
        for node_id in graph_ctx.graph_iter.clone().nodes.iter() {
            let cur_node = graph_ctx.graph.get_node(node_id).unwrap();
            let mut new_inputs = cur_node.inputs.clone();
            for parent_edge in cur_node.edges.input_edges.iter() {
                let parent_node = graph_ctx.graph.get_node(&parent_edge.output_node).unwrap();
                for i in 0..parent_edge.output_idxs.len() {
                    let out_value = &parent_node.outputs[i];
                    new_inputs[parent_edge.input_idxs[i]] = out_value.clone();
                }
            }

            if let Ok(node) = graph_ctx.graph.get_node_mut(node_id) {
                node.node.update(world,render_ctx, &new_inputs, &mut node.outputs);
            }
        }
        
        let command_buffer = render_ctx.command_encoder.take().unwrap().finish();
       
        self.queue.submit(Some(command_buffer));
        render_ctx.resources.clear_swap_chain_texture();
        
        resource::update_mesh_system(world,&mut self.mesh_event_reader,render_ctx);
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
            render_res.create_swap_chain(app_window.width(), app_window.height(), app_window.vsync());
        }
        
    }
}
