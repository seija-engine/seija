use std::path::{PathBuf, Path};
use std::sync::Arc;
use camera::{view_list::view_list_system};
use pipeline::{PipelineCache, update_pipeline_cache};
use render::{AppRender, Config };
pub use script::{builtin_node_creators,RenderScriptContext,RenderScriptPlugin,NodeCreatorSet,NodeCreatorFn};
use seija_app::IModule;
use seija_app::{App};
use bevy_ecs::prelude::*;
pub use render::{RenderGraphContext};
use seija_core::{CoreStage};

extern crate serde_derive;

pub use wgpu;
mod script;
mod graph_setting;
pub mod material;
pub mod camera;
pub mod graph;
pub mod resource;
pub mod pipeline;
pub mod light;

mod uniforms;
mod rt_shaders;
mod mesh_render;
mod render_context;
mod render;
mod memory;
pub use graph_setting::{GraphSetting};
pub use render_context::{RenderContext};
pub use uniforms::{UBOInfoSet,UBOInfo,};
pub use memory::{RawUniformInfo,UniformType,UniformBufferDef,UniformInfo,ArrayPropInfo};



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
    pub plugins:Vec<RenderScriptPlugin>
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
        let render_ctx = RenderContext::new(app_render.device.clone(),&self.0.config_path,self.0.setting.clone());
        self.init_render(w,render_ctx,&mut app_render,config); 
        move |_w| {
            _w.resource_scope(|world:&mut World,mut ctx:Mut<RenderContext>| {
                app_render.update(world,&mut ctx); 
            }); 
        }
    }

    fn init_render(&self,w:&mut World,mut ctx:RenderContext,app_render:&mut AppRender,config:&RenderConfig) {
        let mut rsc = RenderScriptContext::new();
        let creator_set = builtin_node_creators();
        rsc.add_node_creator_set(&creator_set);
        for p in config.plugins.iter() {
            rsc.add_node_creator_set(&p.node_creators);
        }
        let script_path = self.0.config_path.join("render.clj");
        match std::fs::read_to_string(script_path) {
            Ok(code_string) => {
                for p in config.plugins.iter() {
                    match (p.script_mod_name.as_ref(),p.script_mod_code.as_ref()) {
                        (Some(name),Some(code)) => {
                            rsc.rt.add_module(name.as_str(), code.as_str());
                        },
                        _ => {}
                    }
                }
                rsc.run(code_string.as_str(), &mut ctx.ubo_ctx.info,&mut app_render.graph,true);
            },
            Err(err) => {
                log::error!("load render.clj error:{:?}",err);
            }
        }
        app_render.graph.build();
        ctx.ubo_ctx.init(&ctx.device,&mut ctx.resources);
        for node in app_render.graph.graph.iter_mut_nodes() {
            node.node.init(w, &mut ctx);
        }
        w.insert_resource(PipelineCache::default());
        w.insert_resource(ctx);
    }
}