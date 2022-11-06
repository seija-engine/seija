use std::path::PathBuf;

use bevy_ecs::world::World;
use lite_clojure_eval::{EvalRT, Variable};
use lite_clojure_frp::{FRPSystem,fns::add_frp_fns};
use anyhow::Result;
use crate::{UniformInfoSet, RenderContext};

use super::{fns, builder::FRPCompBuilder, frp_comp::{FRPComponent, IElement}, 
            plugin::{RenderScriptPlugin, create_buildin_plugin, NodeCreateFn}, elems::{ElementNode}, errors::Errors};

unsafe impl Send for FRPDSLSystem {}
unsafe impl Sync for FRPDSLSystem {}
pub struct FRPDSLSystem {
    pub vm:EvalRT,
    frp_system:FRPSystem,
    elem_creator:ElementCreator,
    main_comp:Option<FRPComponent>
}

impl FRPDSLSystem {
    pub fn new() -> Self {
        let mut vm = EvalRT::new();
        vm.init();
        add_frp_fns(&mut vm);
        FRPDSLSystem {
            vm, 
            frp_system: FRPSystem::default(),
            main_comp:None,
            elem_creator:ElementCreator::default()
        }
    }

    pub fn init(&mut self,code_string:&str,info_set:&mut UniformInfoSet,lib_paths:&Vec<PathBuf>) {
        let plugin = create_buildin_plugin();
        self.apply_plugin(&plugin);

        for lib_path in lib_paths.iter() {
            self.vm.add_search_path(lib_path);
        }
        fns::init_fns(&mut self.vm);
        self.vm.eval_string(String::default() ,code_string);
        //call init
        let info_ptr = info_set as *mut UniformInfoSet as *mut u8;
        if let Err(err) = self.vm.invoke_func("init", vec![Variable::UserData(info_ptr)]) {
            log::error!("FRPDSLSystem init error:{:?}",err);
        }
    }

    pub fn start(&mut self,ctx:&mut RenderContext,world:&mut World) -> Result<()> {
        

        let mut builder = FRPCompBuilder::new();
        let builder_ptr = &mut builder as *mut FRPCompBuilder as *mut u8;
        self.vm.global_context().set_var("*BUILDER*", Variable::UserData(builder_ptr));
        if let Err(err) = self.vm.invoke_func("start", vec![]) {
            log::error!("FRPDSLSystem start error:{:?}",err);
        }
        let mut main_comp = builder.build(&self.elem_creator)?;
        main_comp.init(world,ctx)?;
        main_comp.active(world,ctx)?;
        self.main_comp = Some(main_comp);
        
        Ok(())
    }

    pub fn apply_plugin(&mut self,plugin:&RenderScriptPlugin) {
        self.elem_creator.apply_plugin(&mut self.vm,plugin);
    }

    pub fn update(&mut self,ctx:&mut RenderContext,world:&mut World) {
        if let Some(main_comp) = self.main_comp.as_mut() {
            main_comp.update(world, ctx);
        }
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