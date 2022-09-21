use std::collections::HashMap;
use std::path::PathBuf;
use bevy_ecs::prelude::{World, Entity, Added, With};
use lite_clojure_eval::{Variable, GcRefCell};
use seija_transform::Transform;
use crate::{camera::camera::Camera};
use crate::{UniformInfoSet, RenderContext, RenderScriptPlugin};
use super::script_plugin::ScriptPlugin;
use super::{ScriptContext, render_path::{RenderPathList}, node::*, builtin::create_builtin_node_set};

//这里通过逻辑保证RenderMain只在一个线程运行，ECS库的System必须要这俩个trait
unsafe impl Send for RenderMain {}
unsafe impl Sync for RenderMain {}


pub struct RenderMain {
    script_ctx:ScriptContext,
    main_ctx:MainContext
}

impl RenderMain {
    pub fn new() -> Self {
        RenderMain { 
            script_ctx:ScriptContext::new(),
            main_ctx:MainContext {
                 plugins:vec![],
                 path_list:RenderPathList::default(),
                 global_env:GcRefCell::new(HashMap::default()),
                 global_nodes:vec![],
                 creators:Default::default()
            }
        }
    }



    pub fn init(&mut self,code_string:&str,lib_paths:&Vec<PathBuf>,info_set:&mut UniformInfoSet) {
        for lib_path in lib_paths.iter() {
            self.script_ctx.rt.add_search_path(lib_path);
        }

        let global_nodes_mut = &mut self.main_ctx.global_nodes;
        let global_node_ptr = global_nodes_mut as *mut Vec<UpdateNodeBox> as *mut u8;
        self.main_ctx.global_env.borrow_mut().insert(Variable::Keyword(GcRefCell::new(":nodes".to_string())), 
                                                     Variable::UserData(global_node_ptr));
        self.add_render_plugin(&Self::create_core_plugin());
        self.script_ctx.init(code_string);
        self.script_ctx.set_userdata("*MAIN_CTX*", &mut self.main_ctx);
        self.script_ctx.exec_declare_uniform(info_set);
        for plugin in self.main_ctx.plugins.iter() {
            plugin.init(&mut self.script_ctx.rt);   
        }
    }

    fn create_core_plugin() -> RenderScriptPlugin {
        let node_sets = create_builtin_node_set();
        let rs = RenderScriptPlugin::new(node_sets);
        rs
    }

    pub fn add_render_plugin(&mut self,plugin:&RenderScriptPlugin) {
        self.add_node_creator(&plugin.node_creators);
        if let Some((mod_name,source)) = plugin.script_mod.as_ref() {
            self.script_ctx.rt.add_module(mod_name.as_str(), source);
        }
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
        for plugin in self.main_ctx.plugins.iter() {
            plugin.start(&mut self.script_ctx.rt); 
        }
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
        let clone_iter:Vec<_> = added_cameras.iter(world).map(|e| (e.0,e.1.path.clone(),e.1.target.clone() )).collect();
        for (e,path,target) in clone_iter {
            self.main_ctx.path_list.add_render_path(&path, &mut self.script_ctx,target,world,ctx,e);   
        }
    }


}

pub struct MainContext {
    pub plugins:Vec<ScriptPlugin>,
    pub global_env:GcRefCell<HashMap<Variable,Variable>>,
    pub global_nodes:Vec<UpdateNodeBox>,

    pub path_list:RenderPathList,

    creators:NodeCreatorContext,
}

impl MainContext {
    pub fn update(&mut self,ctx:&mut RenderContext,world:&mut World,_sc:&mut ScriptContext) {
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

}
