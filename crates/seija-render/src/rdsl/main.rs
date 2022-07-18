use std::collections::HashMap;
use bevy_ecs::prelude::{World, Entity, Added, With};
use lite_clojure_eval::{Variable, GcRefCell};
use seija_app::App;
use seija_asset::Assets;
use seija_core::AddCore;
use seija_transform::Transform;
use crate::{camera::camera::Camera};
use crate::{UniformInfoSet, resource::Texture, RenderContext, RenderScriptPlugin};
use super::{ScriptContext, rt_tags::{RuntimeTags, TagEvent}, render_path::{RenderPathList}, node::*, builtin::create_builtin_node_set};

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

impl RenderMain {
    pub fn new() -> Self {
        RenderMain { 
            script_ctx:ScriptContext::new(),
            main_ctx:MainContext { 
                rt_tags: RuntimeTags::new(),
                 dyn_uniform_set: vec![],
                 path_list:RenderPathList::default(),
                 global_env:GcRefCell::new(HashMap::default()),
                 global_nodes:vec![],
                 creators:Default::default()
            }
        }
    }

    pub fn add_system(app:&mut App) {
        app.add_event::<TagEvent>();
    }

    pub fn init(&mut self,code_string:&str,info_set:&mut UniformInfoSet) {
        let global_nodes_mut = &mut self.main_ctx.global_nodes;
        let global_node_ptr = global_nodes_mut as *mut Vec<UpdateNodeBox> as *mut u8;
        self.main_ctx.global_env.borrow_mut().insert(Variable::Keyword(GcRefCell::new(":nodes".to_string())), 
                                                     Variable::UserData(global_node_ptr));

        self.add_node_creator(&create_builtin_node_set());
        self.script_ctx.init(code_string);
        self.script_ctx.exec_declare_uniform(info_set);
    }

    pub fn add_render_plugin(&mut self,plugin:&RenderScriptPlugin) {
        self.add_node_creator(&plugin.node_creators);
    }

    pub fn add_node_creator(&mut self,set:&NodeCreatorSet) {
        for (k,f) in set.0.iter() {
            let index = self.main_ctx.creators.add(*f) as i64;
            self.script_ctx.rt.global_context().push_var(k.as_str(), Variable::Int(index));
        }
    }

    pub fn start(&mut self,world:&mut World,ctx:&mut RenderContext) {
        self.script_ctx.set_global_const(world);
        self.script_ctx.exec_render_start(ctx,world,&mut self.main_ctx);
        for node_box in self.main_ctx.global_nodes.iter_mut() {
            node_box.set_params(&mut self.script_ctx.rt,true);
            node_box.init(world, ctx);
        }
    }

    

    pub fn update(&mut self,ctx:&mut RenderContext,world:&mut World) {
       self.update_camera(world,ctx);
       self.main_ctx.update(ctx, world,&mut self.script_ctx);
    }

    pub fn update_camera(&mut self,world:&mut World,ctx:&mut RenderContext) {
        let mut added_cameras = world.query_filtered::<(Entity,&Camera),(Added<Camera>,With<Transform>)>();
        if added_cameras.iter(world).count() > 0 {
            self.script_ctx.set_script_global(ctx, &mut self.main_ctx, world);
        }
        for (e,add_camera) in added_cameras.iter(world) {
            self.main_ctx.path_list.add_render_path(&add_camera.path, &mut self.script_ctx,add_camera,world,ctx,e);   
        }
    }


}

pub struct MainContext {
    pub rt_tags:RuntimeTags,
    pub dyn_uniform_set:Vec<DynUniformItem>,
    pub global_env:GcRefCell<HashMap<Variable,Variable>>,
    pub global_nodes:Vec<UpdateNodeBox>,

    pub path_list:RenderPathList,

    creators:NodeCreatorContext,
}

impl MainContext {
    pub fn update(&mut self,ctx:&mut RenderContext,world:&mut World,sc:&mut ScriptContext) {
        self.rt_tags.update(world);
        if self.rt_tags.dirtys.len() > 0 {
            self.update_dirty_tag(ctx);
        }

        for node in self.global_nodes.iter_mut() {
            node.prepare(world, ctx);
        }
        for node in self.global_nodes.iter_mut() {
            node.update(world, ctx);
        }

        self.path_list.update(world, ctx);
    }

    

    pub fn create_node(&mut self,index:usize,params:Vec<Variable>) -> Option<UpdateNodeBox> {
        let f = self.creators.creators.get(index)?;
        let node = f(self,params);
        Some(node)
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
        for node in self.global_nodes.iter_mut() {
            node.update_enable(&self.rt_tags);
        }
        self.rt_tags.dirtys.clear();
    }
}
