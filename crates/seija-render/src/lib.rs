use std::path::{PathBuf, Path};
use std::sync::Arc;
use material::MaterialStorage;
use pipeline::{PipelineCache, update_pipeline_cache};
use rdsl::{RenderMain};
use render::{AppRender, Config };
use resource::Mesh;
use resource::shape::{Cube, Sphere, Plane, Quad};
use seija_app::IModule;
use seija_app::{App};
use bevy_ecs::prelude::*;
use seija_asset::{AssetServer, Assets};
use seija_core::{CoreStage};
extern crate serde_derive;
pub use wgpu;
mod graph_setting;
pub mod rdsl;
pub mod material;
pub mod camera;
pub mod resource;
pub mod pipeline;
pub mod light;
pub mod errors;
pub mod shadow;
mod query;

mod uniforms;
mod rt_shaders;
mod mesh_render;
mod render_context;
mod render;
mod memory;
pub use rdsl::{RenderScriptPlugin,NodeCreatorFn,NodeCreatorSet,ScriptContext,IUpdateNode,UpdateNodeBox,UBOArrayCollect};
pub use graph_setting::{GraphSetting};
pub use render_context::{RenderContext};
pub use uniforms::{UniformInfoSet,UniformInfo,UniformIndex};
pub use uniforms::backends::{IShaderBackend};
pub use memory::{UniformInfo as MemUniformInfo,RawUniformInfo,UniformType,UniformBufferDef,UniformBuffer,ArrayPropInfo};


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
    pub render_lib_paths:Vec<PathBuf>
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
        light::init_light(app);
        RenderMain::add_system(app);
        query::init_system(app);
        Self::init_buildin_assets(&mut app.world);

        let render_system = self.get_render_system(&mut app.world,self.0.clone());
        app.schedule.add_stage_after(CoreStage::PostUpdate, RenderStage::AfterRender, SystemStage::parallel());
        app.schedule.add_stage_before(RenderStage::AfterRender, RenderStage::Render, SystemStage::single(render_system.exclusive_system()));
        app.schedule.add_stage_before(RenderStage::Render, RenderStage::PostRender, SystemStage::parallel());

        
        app.add_system(RenderStage::AfterRender, update_pipeline_cache);
    }
}


impl RenderModule {
    fn get_render_system(&self,w:&mut World,config:Arc<RenderConfig>) -> impl FnMut(&mut World) {
        let mut app_render = AppRender::new_sync(Config::default());
        let mut render_ctx = RenderContext::new(app_render.device.clone(),&self.0.config_path,self.0.setting.clone());
        //TODO 这里考虑把MaterialStorage的默认贴图删了
        render_ctx.resources.default_textures = w.get_resource::<MaterialStorage>().unwrap().default_textures.clone();
        self.init_render(w,render_ctx,&mut app_render,config); 
        move |_w| {
            _w.resource_scope(|world:&mut World,mut ctx:Mut<RenderContext>| {
                app_render.update(world,&mut ctx); 
            }); 
        }
    }

    fn init_render(&self,w:&mut World,mut ctx:RenderContext,app_render:&mut AppRender,config:Arc<RenderConfig>) {
        for plugin in self.0.plugins.iter() {
            app_render.main.add_render_plugin(plugin);
        }
        w.insert_resource(PipelineCache::new(config.clone()));
        ctx.ubo_ctx.init(&mut ctx.resources);
        
      
        match std::fs::read_to_string(&self.0.script_path) {
            Ok(code_string) => {
                app_render.main.init(&code_string,&config.render_lib_paths,&config.config_path,&mut ctx.ubo_ctx.info);
            },
            Err(err) => {
                log::error!("load main render script:{:?} error:{:?}",&self.0.script_path,err);
            }
        }

        app_render.main.start(w, &mut ctx);
        w.insert_resource(ctx);       
    }

    fn init_buildin_assets(world:&mut World) {
        let mut meshs = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let h_cube = meshs.add(Cube::new(1f32).into());
        let h_sphere = meshs.add(Sphere::new(0.5f32).into());
        let h_plane = meshs.add(Plane::new(10f32,10).into());
        let h_quad = meshs.add(Quad::new(1f32).into());
        if let Some(assets) = world.get_resource::<AssetServer>() {
            assets.set_asset("buildin#cube", h_cube.untyped());
            assets.set_asset("buildin#sphere", h_sphere.untyped());
            assets.set_asset("buildin#plane", h_plane.untyped());
            assets.set_asset("buildin#quad", h_quad.untyped());
        }
    }
}