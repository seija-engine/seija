use lite_clojure_eval::{Variable, EvalRT};

pub trait IUpdateNode {
    fn update_params(&self,params:Vec<Variable>);
}

pub struct UpdateNodeBox {
    pub params:Vec<Variable>,
    pub node:Box<dyn IUpdateNode>
}

impl UpdateNodeBox {
    pub fn create<T>(params:&Vec<Variable>) -> UpdateNodeBox where T:Default + IUpdateNode + 'static {
        let node:T = Default::default();
        UpdateNodeBox { params:params.clone(), node:Box::new(node) }
    }

    pub fn set_params(&mut self,rt:&mut EvalRT,is_first:bool) {
        if !is_first {
            if !self.params.iter().any(|v| match v {
                Variable::Function(_) => true,
                _ => false
            }) { return; }
        }

        let mut new_params:Vec<Variable> = vec![];
        for param in self.params.iter() {
            match param {
                Variable::Function(_) => { 
                    match rt.invoke_func2(param, vec![]) {
                        Ok(v) => { new_params.push(v); },
                        Err(err) => { log::error!("{:?}",err); }
                    }
                 },
                _ => {
                    new_params.push(param.clone());
                }
            }
        }
        self.node.update_params(new_params);
    }
}