use std::path::{PathBuf, Path};
use camera::{view_list::view_list_system};
use pipeline::{PipelineCache, update_pipeline_cache};
use render::{AppRender, Config };
use script::{RenderScriptContext, builtin_node_creators};
use seija_app::IModule;
use seija_app::{App};
use bevy_ecs::prelude::*;
use seija_core::{CoreStage};
#[macro_use]
extern crate serde_derive;

pub use wgpu;
mod script;
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

pub use render_context::{RenderContext};
pub use uniforms::{UBOInfoSet,UBOInfo};


#[derive(Debug, Hash, PartialEq, Eq, Clone,StageLabel )]
pub enum RenderStage {
    AfterRender,
    Render,
    PostRender,
}

#[derive(Default)]
pub struct RenderConfig {
    config_path:PathBuf
}

impl RenderConfig {
    pub fn set_config_path<P:AsRef<Path>>(&mut self,path:P) {
        self.config_path = path.as_ref().into();
    }
}

#[derive(Default)]
pub struct RenderModule(pub RenderConfig);

impl IModule for RenderModule {
    fn init(&mut self,app:&mut App) {
        
        resource::init_resource(app);
        material::init_material(app);
        light::init_light(app);

        let render_system = self.get_render_system(&mut app.world);
        app.schedule.add_stage_after(CoreStage::PostUpdate, RenderStage::AfterRender, SystemStage::parallel());
        app.schedule.add_stage_before(RenderStage::AfterRender, RenderStage::Render, SystemStage::single(render_system.exclusive_system()));
        app.schedule.add_stage_before(RenderStage::Render, RenderStage::PostRender, SystemStage::parallel());

        

        app.add_system(RenderStage::AfterRender, update_pipeline_cache.system());
        app.add_system(CoreStage::PostUpdate, view_list_system.system());
    }
}


impl RenderModule {
    fn get_render_system(&self,w:&mut World) -> impl FnMut(&mut World) {
        let mut app_render = AppRender::new_sync(Config::default());
        let render_ctx = RenderContext::new(app_render.device.clone(),&self.0.config_path);
        self.init_render(w,render_ctx,&mut app_render); 
        move |_w| {
            _w.resource_scope(|world:&mut World,mut ctx:Mut<RenderContext>| {
                app_render.update(world,&mut ctx); 
            }); 
        }
    }

    fn init_render(&self,w:&mut World,mut ctx:RenderContext,app_render:&mut AppRender) {
        let mut rsc = RenderScriptContext::new();
        let creator_set = builtin_node_creators();
        rsc.add_node_creator_set(&creator_set);
        let script_path = self.0.config_path.join("render.clj");
        match std::fs::read_to_string(script_path) {
            Ok(code_string) => {
                rsc.run(code_string.as_str(), &mut ctx.ubo_ctx.info,&mut app_render.graph);
            },
            Err(err) => {
                log::error!("load render.clj error:{:?}",err);
            }
        }
        app_render.graph.build();
        ctx.ubo_ctx.init();
        for node in app_render.graph.graph.iter_mut_nodes() {
            node.node.init(w, &mut ctx);
        }
        w.insert_resource(PipelineCache::default());
        w.insert_resource(ctx);
    }
}