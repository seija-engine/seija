use std::path::PathBuf;

use bevy_ecs::world::World;
use lite_clojure_eval::{EvalRT, Variable};
use lite_clojure_frp::{FRPSystem,fns::add_frp_fns};
use anyhow::Result;
use crate::{UniformInfoSet, RenderContext};

use super::{fns, builder::FRPCompBuilder, frp_comp::{FRPComponent, IElement}};

unsafe impl Send for FRPDSLSystem {}
unsafe impl Sync for FRPDSLSystem {}
pub struct FRPDSLSystem {
    vm:EvalRT,
    frp_system:FRPSystem,
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
            main_comp:None
        }
    }

    pub fn init(&mut self,code_string:&str,info_set:&mut UniformInfoSet,lib_paths:&Vec<PathBuf>) {
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
        let mut main_comp = builder.build()?;
        main_comp.active(ctx);
        self.main_comp = Some(main_comp);
        
        Ok(())
    }

    pub fn update(&mut self,ctx:&mut RenderContext,world:&mut World) {
        
    }
}


#[test]
fn test_system() {
    env_logger::init();
    let mut system = FRPDSLSystem::new();
    system.init(r#"
    (defn init [set]
       (println "init exec")
    )
    
    (defn start []
      (__frp_enter__ "start")
      (uniform "ObjectBuffer")

      (__frp_exit__)
    )
    
    
    "#, &mut UniformInfoSet::default(),&vec![]);
    
}