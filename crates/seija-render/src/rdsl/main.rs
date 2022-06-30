use std::collections::HashMap;

use bevy_ecs::prelude::World;
use seija_app::App;
use seija_asset::Assets;
use seija_core::AddCore;
use crate::{UniformInfoSet, resource::Texture, RenderContext};
use super::{ScriptContext, rt_tags::{RuntimeTags, TagEvent}, render_path::RenderPathDef, node::IUpdateNode};

//这里通过逻辑保证RenderMain只在一个线程运行，ECS库的System必须要这俩个trait
unsafe impl Send for RenderMain {}
unsafe impl Sync for RenderMain {}

pub struct DynUniformItem {
    pub tag_index:usize,
    pub enable:bool,
    pub ubo_name:String
}

pub struct RenderMain {
    script_ctx:ScriptContext,
    main_ctx:MainContext
}

pub struct MainContext {
    pub rt_tags:RuntimeTags,
    pub dyn_uniform_set:Vec<DynUniformItem>,
    pub path_dic:HashMap<String,RenderPathDef>,

    pub global_nodes:Vec<Box<dyn IUpdateNode>>,
}

impl RenderMain {
    pub fn new() -> Self {
        RenderMain { 
            script_ctx:ScriptContext::new(),
            main_ctx:MainContext { 
                rt_tags: RuntimeTags::new(),
                 dyn_uniform_set: vec![],
                 path_dic:HashMap::default(),
                 global_nodes:vec![]
            }
        }
    }

    pub fn add_system(app:&mut App) {
        app.add_event::<TagEvent>();
    }

    pub fn init(&mut self,code_string:&str,info_set:&mut UniformInfoSet) {
        self.script_ctx.init(code_string);
        self.script_ctx.exec_declare_uniform(info_set);
    }

    pub fn start(&mut self,textures:&mut Assets<Texture>,ctx:&mut RenderContext) {
        self.script_ctx.exec_render_start(ctx, textures,&mut self.main_ctx);
        self.script_ctx.exec_render_update(ctx, textures, &mut self.main_ctx);
    }

    pub fn update(&mut self,ctx:&mut RenderContext,world:&mut World) {
       self.main_ctx.update(ctx, world);
    }

}


impl MainContext {
    pub fn update(&mut self,ctx:&mut RenderContext,world:&mut World) {
        self.rt_tags.update(world);
        if self.rt_tags.dirtys.len() > 0 {
            self.update_dirty_tag(ctx);
        }
    }

    fn update_dirty_tag(&mut self,ctx:&mut RenderContext) {
        for info in self.dyn_uniform_set.iter_mut() {
            if self.rt_tags.dirtys.contains(&info.tag_index) && self.rt_tags.tags[info.tag_index] != info.enable {
                if info.enable {
                    ctx.ubo_ctx.remove_uniform(&info.ubo_name);
                } else {
                    ctx.ubo_ctx.add_uniform(&info.ubo_name, &mut ctx.resources);
                }
                info.enable = !info.enable;
            }
        }
        self.rt_tags.dirtys.clear();
    }
}
