use lite_clojure_eval::{EvalRT, Variable};
use lite_clojure_frp::{FRPSystem,fns::add_frp_fns};

use crate::UniformInfoSet;

use super::{fns, builder::FRPCompBuilder};
pub struct FRPDSLSystem {
    vm:EvalRT,
    frp_system:FRPSystem,
}

impl FRPDSLSystem {
    pub fn new() -> Self {
        let mut vm = EvalRT::new();
        vm.init();
        add_frp_fns(&mut vm);
        FRPDSLSystem { 
            vm, 
            frp_system: FRPSystem::default() 
        }
    }

    pub fn init(&mut self,code_string:&str,info_set:&mut UniformInfoSet) {
        fns::init_fns(&mut self.vm);
        self.vm.eval_string(String::default() ,code_string);
        //call init
        let info_ptr = info_set as *mut UniformInfoSet as *mut u8;
        if let Err(err) = self.vm.invoke_func("init", vec![Variable::UserData(info_ptr)]) {
            log::error!("FRPDSLSystem init error:{:?}",err);
        }
    }

    pub fn start(&mut self) {
        let mut builder = FRPCompBuilder::new();
        let builder_ptr = &mut builder as *mut FRPCompBuilder as *mut u8;
        self.vm.global_context().set_var("*BUILDER*", Variable::UserData(builder_ptr));
        if let Err(err) = self.vm.invoke_func("start", vec![]) {
            log::error!("FRPDSLSystem start error:{:?}",err);
        }
        builder.build();
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
    )
    
    
    "#, &mut UniformInfoSet::default());
    system.start();
}