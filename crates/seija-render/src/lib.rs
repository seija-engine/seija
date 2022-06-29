use std::path::{PathBuf, Path};
use std::sync::Arc;
use camera::{view_list::view_list_system};
use material::MaterialStorage;
use pipeline::{PipelineCache, update_pipeline_cache};
use rdsl::RenderMain;
use render::{AppRender, Config };
use resource::Texture;
pub use script::{builtin_node_creators,RenderScriptContext,RenderScriptPlugin,NodeCreatorSet,NodeCreatorFn};
use seija_app::IModule;
use seija_app::{App};
use bevy_ecs::prelude::*;
pub use render::{RenderGraphContext};
use seija_asset::Assets;
use seija_core::{CoreStage};

extern crate serde_derive;

pub use wgpu;
mod script;
mod graph_setting;
pub mod rdsl;
pub mod material;
pub mod camera;
pub mod graph;
pub mod resource;
pub mod pipeline;
pub mod light;
pub mod errors;

mod uniforms;
mod rt_shaders;
mod mesh_render;
mod render_context;
mod render;
mod memory;
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

        let render_system = self.get_render_system(&mut app.world,&self.0);
        app.schedule.add_stage_after(CoreStage::PostUpdate, RenderStage::AfterRender, SystemStage::parallel());
        app.schedule.add_stage_before(RenderStage::AfterRender, RenderStage::Render, SystemStage::single(render_system.exclusive_system()));
        app.schedule.add_stage_before(RenderStage::Render, RenderStage::PostRender, SystemStage::parallel());

        

        app.add_system(RenderStage::AfterRender, update_pipeline_cache.system());
        app.add_system(CoreStage::PostUpdate, view_list_system.system());
    }
}


impl RenderModule {
    fn get_render_system(&self,w:&mut World,config:&RenderConfig) -> impl FnMut(&mut World) {
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

    fn init_render(&self,w:&mut World,mut ctx:RenderContext,app_render:&mut AppRender,config:&RenderConfig) {
       
        w.insert_resource(PipelineCache::default());
        ctx.ubo_ctx.init(&mut ctx.resources);
        let script_path = self.0.config_path.join("new_render.clj");
        match std::fs::read_to_string(script_path) {
            Ok(code_string) => {
                app_render.main.init(&code_string,&mut ctx.ubo_ctx.info);
            },
            Err(err) => {
                log::error!("load render.clj error:{:?}",err);
            }
        }

        let mut textures = w.get_resource_mut::<Assets<Texture>>().unwrap();
        let textures_mut:&mut Assets<Texture> = &mut textures;

        app_render.main.start(textures_mut, &mut ctx);
        w.insert_resource(ctx);
       
    }
}