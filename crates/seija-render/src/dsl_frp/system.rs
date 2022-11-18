use std::path::PathBuf;
use seija_app::ecs::{world::World,change_detection::Mut};
use lite_clojure_eval::{EvalRT, Variable};
use lite_clojure_frp::{FRPSystem,fns::add_frp_fns};
use anyhow::Result;
use crate::{UniformInfoSet, RenderContext, frp_context::{FRPContext, FRPContextInner}};

use super::{fns, builder::FRPCompBuilder, frp_comp::{IElement, FRPComponent}, 
            plugin::{RenderScriptPlugin, create_buildin_plugin, NodeCreateFn}, 
            elems::{ElementNode}, errors::Errors, render_path::{RenderPathContext}};

unsafe impl Send for FRPDSLSystem {}
unsafe impl Sync for FRPDSLSystem {}
pub struct FRPDSLSystem {
    pub vm:EvalRT,
    elem_creator:ElementCreator,
    path_context:RenderPathContext,
    main_comp:Option<FRPComponent>
}

impl FRPDSLSystem {
    pub fn new() -> Self {
        let mut vm = EvalRT::new();
        vm.init();
        add_frp_fns(&mut vm);
        FRPDSLSystem {
            vm,
            path_context:RenderPathContext::default(),
            main_comp:None,
            elem_creator:ElementCreator::default()
        }
    }

    pub fn init(&mut self,code_string:&str,info_set:&mut UniformInfoSet,lib_paths:&Vec<PathBuf>,frp_ctx:Option<&FRPContext>) {
        let plugin = create_buildin_plugin();
        self.apply_plugin(&plugin,frp_ctx);

        for lib_path in lib_paths.iter() {
            self.vm.add_search_path(lib_path);
        }
        fns::init_fns(&mut self.vm);
        let path_ctx_ptr = &mut self.path_context as *mut RenderPathContext as *mut u8;
        self.vm.global_context().set_var("*PATH_DEFINE_SET*", Variable::UserData(path_ctx_ptr));
        self.vm.eval_string(String::default() ,code_string);
        //call init
        let info_ptr = info_set as *mut UniformInfoSet as *mut u8;
        if let Err(err) = self.vm.invoke_func("init", vec![Variable::UserData(info_ptr)]) {
            log::error!("FRPDSLSystem init error:{:?}",err);
        }
    }

    pub fn start(&mut self,ctx:&mut RenderContext,world:&mut World) -> Result<()> {
        world.resource_scope(|world:&mut World,frp_ctx:Mut<FRPContext>| {
            let mut builder = FRPCompBuilder::new();
            let mut write_frp = frp_ctx.inner.write();
            let mut_system:&mut FRPSystem = &mut write_frp.system;
            Self::set_global_vars(&mut self.vm, &mut builder, world, mut_system);
            if let Err(err) = self.vm.invoke_func("start", vec![]) {
                log::error!("FRPDSLSystem start error:{:?}",err);
            }
            let mut main_comp = builder.build(&self.elem_creator,&mut self.vm)?;
            main_comp.init(world,ctx,mut_system,&mut self.vm,&self.elem_creator)?;
            main_comp.active(world,ctx,mut_system)?;
            self.main_comp = Some(main_comp);
        
            Ok(())
        })
    }

    pub fn apply_plugin(&mut self,plugin:&RenderScriptPlugin,frp_ctx:Option<&FRPContext>) {
        self.elem_creator.apply_plugin(&mut self.vm,plugin);
        self.path_context.apply_plugin(plugin, frp_ctx);
        if let Some(frp_ctx) = frp_ctx {
            let mut ctx_inner = frp_ctx.inner.write();
            for event in plugin.global_events.iter() {
               let evid = ctx_inner.new_event(Some(event.clone()));
               self.vm.global_context().set_var(event.as_str(), Variable::Int(evid as i64));
            }
            
            for (name,default_value) in plugin.global_dynamics.iter() {
                let dynid = ctx_inner.new_dynamic(Some(name.clone()),default_value.clone());
                self.vm.global_context().set_var(name.as_str(), Variable::Int(dynid as i64));
            }

            
        }
    }

    pub fn update(&mut self,ctx:&mut RenderContext,world:&mut World) {
        world.resource_scope(|world:&mut World,frp_ctx:Mut<FRPContext>| {
            let mut write_frp = frp_ctx.inner.write();
            let mut_ctx_inner:&mut FRPContextInner = &mut write_frp;
            if let Some(main_comp) = self.main_comp.as_mut() {
                main_comp.update(world, ctx,&mut mut_ctx_inner.system);
            }
            self.path_context.update(world, ctx,&self.elem_creator,&mut self.vm,mut_ctx_inner);
        });
    }

    pub fn set_global_vars(vm:&mut EvalRT,builder:&mut FRPCompBuilder,world:&mut World,system:&mut FRPSystem) {
        let builder_ptr = builder as *mut FRPCompBuilder as *mut u8;
        vm.global_context().set_var("*BUILDER*", Variable::UserData(builder_ptr));
        let world_ptr = world as *mut World as *mut u8;
        vm.global_context().set_var("*WORLD*", Variable::UserData(world_ptr));
        let frp_ptr = system as *mut FRPSystem as *mut u8;
        vm.global_context().set_var("*FRPSystem*", Variable::UserData(frp_ptr));
    }

}

#[derive(Default)]
pub struct ElementCreator {
    node_creators:Vec<NodeCreateFn>
}

impl ElementCreator {
    pub fn apply_plugin(&mut self,vm:&mut EvalRT,plugin:&RenderScriptPlugin) {
        for (name,f) in plugin.node_creators.iter() {
            let var_name = format!("{}NodeID",name.as_str());
            self.node_creators.push(*f);
            let index = (self.node_creators.len() - 1) as i64;
            vm.global_context().set_var(var_name.as_str(), Variable::Int(index));
        }
    }

    pub fn create_node(&self,index:usize,args:Vec<Variable>) -> Result<ElementNode> {
        let creatorf = self.node_creators.get(index).ok_or(Errors::NotFoundNodeCreator)?;
        let node = creatorf(args)?;
        let node = ElementNode::new(node);
        Ok(node)
    }
}