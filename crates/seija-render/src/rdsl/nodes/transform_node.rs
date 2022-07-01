use lite_clojure_eval::Variable;
use crate::rdsl::{node::IUpdateNode};
#[derive(Default)]
pub struct TransfromNode {
    
}

impl IUpdateNode for TransfromNode {
    fn update_params(&self,params:Vec<Variable>) {
        dbg!(&params);
    }
}