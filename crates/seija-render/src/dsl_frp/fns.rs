use lite_clojure_eval::{EvalRT, ExecScope, Variable, run_native_fn};
use serde_json::Value;
use thiserror::Error;
use anyhow::anyhow;
use std::{convert::TryFrom, sync::Arc};
use crate::{UniformInfo, UniformInfoSet};

#[derive(Debug,Error)]

enum Errors {
    #[error("not found info set")]
    NotFoundInfoSet,
    #[error("type cast error {0}")]
    TypeCastError(&'static str)
}

pub fn init_fns(vm:&mut EvalRT) {
    vm.global_context().push_native_fn("declare-uniform", declare_uniform);
}

pub fn declare_uniform(s:&mut ExecScope,a:Vec<Variable>) -> Variable { 
    run_native_fn("declare-uniform", s, a, |_,mut args| {
        let info_set = args[0].cast_userdata().ok_or(Errors::NotFoundInfoSet)?;
        let name:String = args[1].cast_string().ok_or(Errors::TypeCastError("string"))?.borrow().clone();
        let json_value:Value = args.remove(2).into();
        let mut info = UniformInfo::try_from(&json_value).map_err(|e| anyhow!(e))?;
        info.name =  Arc::new(name);
        let info_set_mut = unsafe { &mut *(info_set as *mut UniformInfoSet) };
        info_set_mut.add_info(info);
        Ok(Variable::Nil)
    })
}