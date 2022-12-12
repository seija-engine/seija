use std::path::{PathBuf, Path};
use std::sync::Arc;
use camera::camera_frp_event_system;
use dsl_frp::RenderScriptPlugin;
use query::QuerySystem;
use render::{AppRender, Config };
use frp_context::FRPContext;
use resource::{Mesh, Texture, color_texture,cube_texture};
use resource::shape::{Cube, Sphere, Plane, Quad, SkyBox};
use scene::SceneEnv;
use seija_app::IModule;
use seija_app::{App};
use bevy_ecs::prelude::*;
use seija_asset::{AssetServer, Assets};
use seija_core::{CoreStage};
extern crate serde_derive;
pub use wgpu;
mod frp_context;
mod graph_setting;
pub mod dsl_frp;
pub mod material;
pub mod camera;
pub mod resource;
pub mod pipeline;
pub mod light;
pub mod errors;
pub mod shadow;
mod query;
pub mod scene;
pub mod ffi;
mod uniforms;
mod rt_shaders;
mod mesh_render;
mod render_context;
mod render;
mod memory;
pub use graph_setting::GraphSetting;
pub use render_context::RenderContext;
pub use uniforms::{UniformInfoSet,UniformInfo,UniformIndex};
pub use uniforms::backends::IShaderBackend;
pub use memory::{UniformInfo as MemUniformInfo,RawUniformInfo,UniformType,UniformBufferDef,UniformBuffer,ArrayPropInfo};
pub use query::SceneOctreeModule;


#[derive(Debug, Hash, PartialEq, Eq, Clone,StageLabel )]
pub enum RenderStage {
    AfterRender,
    Render,
    PostRender,
}

#[derive(Default)]
pub struct RenderConfig {
    pub config_path:PathBuf,
    pub script_path:PathBuf,
    pub setting:Arc<GraphSetting>,
    pub plugins:Vec<RenderScriptPlugin>,
    pub render_lib_paths:Vec<PathBuf>,
    pub pre_render_updates:Vec<fn(world:&mut World,ctx:&mut RenderContext)>
}

impl RenderConfig {
    pub fn set_config_path<P:AsRef<Path>>(&mut self,path:P) {
        self.config_path = path.as_ref().into();
    }
}

#[derive(Default)]
pub struct RenderModule(pub Arc<RenderConfig>);

impl IModule for RenderModule {
    fn init(&mut self,app:&mut App) {
        resource::init_resource(app);
        material::init_material(app);
        dsl_frp::init_dsl_frp(app);
        light::init_light(app);
        shadow::init_shadow(app);

        
        app.add_resource(FRPContext::new());
        Self::init_buildin_assets(&mut app.world);
        app.add_resource(QuerySystem::default());
        let render_system = self.get_render_system(&mut app.world,self.0.clone());
        app.schedule.add_stage_after(CoreStage::PostUpdate, RenderStage::AfterRender, SystemStage::parallel());
        app.schedule.add_stage_before(RenderStage::AfterRender, RenderStage::Render, SystemStage::single(render_system.exclusive_system()));
        app.schedule.add_stage_before(RenderStage::Render, RenderStage::PostRender, SystemStage::parallel());
        query::init_system(app);

        app.add_system(CoreStage::PostUpdate, camera_frp_event_system);
        app.init_resource::<SceneEnv>();
    }
}


impl RenderModule {
    fn get_render_system(&self,w:&mut World,config:Arc<RenderConfig>) -> impl FnMut(&mut World) {
        let mut app_render = AppRender::new_sync(Config::default());
        app_render.pre_render_updates = config.pre_render_updates.clone();
        let assets = w.get_resource::<AssetServer>().unwrap();
        let render_ctx = RenderContext::new(app_render.device.clone(),self.0.clone(),assets);
       
        self.init_render(w,render_ctx,&mut app_render,config); 
        move |_w| {
            _w.resource_scope(|world:&mut World,mut ctx:Mut<RenderContext>| {
                app_render.update(world,&mut ctx); 
            }); 
        }
    }

    fn init_render(&self,w:&mut World,mut ctx:RenderContext,app_render:&mut AppRender,config:Arc<RenderConfig>) {
        let frp_ctx = w.get_resource::<FRPContext>().unwrap();
        for plugin in self.0.plugins.iter() {
            app_render.frp_render.apply_plugin(plugin,Some(frp_ctx));
        }
        ctx.ubo_ctx.init(&mut ctx.resources);
        
        match std::fs::read_to_string(&self.0.script_path) {
            Ok(code_string) => {        
                app_render.frp_render.init(&code_string,&mut ctx.ubo_ctx.info,&config.render_lib_paths,Some(frp_ctx));
            },
            Err(err) => {
                log::error!("load main render script:{:?} error:{:?}",&self.0.script_path,err);
            }
        }


        if let Err(err) = app_render.frp_render.start(&mut ctx,w) {
            log::error!("{:?} start error:{:?}",&self.0.script_path,err);
        }
        w.insert_resource(ctx);       
    }

    fn init_buildin_assets(world:&mut World) {
        let mut meshs = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        
        let h_cube = meshs.add_weak(Cube::new(1f32).into());
        let h_sphere = meshs.add_weak(Sphere::new(0.5f32).into());
        let h_plane = meshs.add_weak(Plane::new(10f32,10).into());
        let h_quad = meshs.add_weak(Quad::new(1f32).into());
        let h_skybox = meshs.add_weak(SkyBox.into());
        let h_quad2 = meshs.add_weak(Quad::new(2f32).into());

        let mut textures = world.get_resource_mut::<Assets<Texture>>().unwrap();
        let h_white = textures.add_weak(color_texture([255,255,255,255], 16));
        let h_blue  = textures.add_weak(color_texture([0,0,255,255], 16));
        let h_black = textures.add_weak(color_texture([0,0,0,255], 16));
        let h_cubemap  = textures.add_weak(cube_texture([255,255,255,255], 16));
        
        if let Some(assets) = world.get_resource::<AssetServer>() {
            assets.set_asset("mesh:cube", h_cube.id);
            assets.set_asset("mesh:sphere", h_sphere.id);
            assets.set_asset("mesh:plane", h_plane.id);
            assets.set_asset("mesh:quad", h_quad.id);
            assets.set_asset("mesh:skybox", h_skybox.id);
            assets.set_asset("mesh:quad2", h_quad2.id);

            assets.set_asset("texture:white", h_white.id);
            assets.set_asset("texture:blue", h_blue.id);
            assets.set_asset("texture:black", h_black.id);
            assets.set_asset("texture:cube", h_cubemap.id);
        }
    }
}