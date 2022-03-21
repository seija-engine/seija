use std::{convert::TryFrom, sync::Arc, collections::HashMap};
use lite_clojure_parser::{ast::{parse_ast, ASTModule}, expr::Expr};
use lite_clojure_eval::{EvalRT, Variable};
use seija_render::UBOInfo;
use serde_json::Value;
pub struct RenderInfo {
    rt:EvalRT,
    ubos:Vec<Arc<UBOInfo>>,
    pub backend2ubo:HashMap<String,Arc<UBOInfo>>,
}

impl RenderInfo {
    pub fn new() -> Self {
        RenderInfo { rt:EvalRT::new(),ubos:vec![],backend2ubo:HashMap::default() }
    }

    pub fn init(&mut self) {
        self.rt.init();
        self.rt.push_native_fn("def-ubo", def_ubo);

        let this_ptr = self as *mut RenderInfo as *mut u8;
        self.rt.push_var("THIS", Variable::UserData(this_ptr));
    }

    pub fn run(&mut self,path:&str) {
       match std::fs::read_to_string(path) {
           Ok(code) => {
             match parse_ast(path.to_string(),&code) {
                 Ok(ast_module) => {
                    let new_ast_module = filter_ast_module(ast_module);
                    self.rt.eval_ast_module(new_ast_module);
                 },
                 Err(err) => {
                     log::error!("{:?}",err);
                 }
             }
           },
           Err(err) => {
               log::error!("{:?}",err);
           }
       }
    }

    fn add_ubo_info(&mut self,info:UBOInfo) {
        let arc_info = Arc::new(info);
        self.ubos.push(arc_info.clone());
        for backend in arc_info.backends.iter() {
            self.backend2ubo.insert(backend.clone(), arc_info.clone());
        }
    } 
}


fn def_ubo(rt:&mut EvalRT,mut args:Vec<Variable>) -> Variable {
    let ptr = rt.find_symbol("THIS").and_then(|v| v.cast_userdata());
    if ptr.is_none() {
        log::error!("not found THIS");
        return Variable::Nil;
    }
    let info = unsafe { &mut *(ptr.unwrap() as *mut RenderInfo)  };
    let json:Value = args.remove(0).into();
    let eubo_info = UBOInfo::try_from(&json);
    if let Err(err) = eubo_info.as_ref() {
        log::error!("def-ubo error:{:?}",err);
    }
    info.add_ubo_info(eubo_info.unwrap());
    Variable::Nil
}


fn filter_ast_module(ast_module:ASTModule) -> ASTModule {
    let mut new_exprs:Vec<Expr> = vec![];
    for expr in ast_module.exprs {
        match expr {
            Expr::Invoke(lst) => {
                match &lst[0] {
                    Expr::Symbol(s) if s.name == "def-ubo" => {
                        new_exprs.push(Expr::Invoke(lst));
                    }
                    _ => {}
                }
            },
            _ => {}
        }
    }
    ASTModule { file_name: ast_module.file_name, exprs: new_exprs }
}
