use lite_clojure_eval::{Variable, EvalRT, GcRefCell};
use anyhow::{Result,anyhow};

pub struct ScriptPlugin {
    init_fn:Variable,
    start_fn:Variable,
    exit_fn:Option<Variable>
}

impl ScriptPlugin {
    pub fn new(var:Variable) -> Result<Self> {
        let object = var.cast_map().ok_or(anyhow!("plugin"))?;
        let init_key = Variable::String(GcRefCell::new("init".to_string()));
        let start_key = Variable::String(GcRefCell::new("start".to_string()));
        let exit_key = Variable::String(GcRefCell::new("exit".to_string()));
        let init_fn = object.borrow_mut().remove(&init_key).ok_or(anyhow!("plugin get init error"))?;
        let start_fn = object.borrow_mut().remove(&start_key).ok_or(anyhow!("plugin get start error"))?;
        let exit_fn = object.borrow_mut().remove(&exit_key);
        Ok(ScriptPlugin {
            init_fn,
            start_fn,
            exit_fn
        })
    }

    pub fn init(&self,eval:&mut EvalRT) {
       if let Err(err) = eval.invoke_func2(&self.init_fn, vec![]) {
        log::error!("plugin run init error {:?}",err);
       }
    }

    pub fn start(&self,eval:&mut EvalRT) {
        if let Err(err) = eval.invoke_func2(&self.start_fn, vec![]) {
         log::error!("plugin run start error {:?}",err);
        }
     }

     pub fn exit(&self,eval:&mut EvalRT) {
        if let Some(exit_fn) = self.exit_fn.as_ref() {
            if let Err(err) = eval.invoke_func2(exit_fn, vec![]) {
                log::error!("plugin run start error {:?}",err);
            }
        }
     }
}