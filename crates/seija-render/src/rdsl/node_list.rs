use lite_clojure_eval::Variable;

pub struct ScriptNodeList {
   pub fn_var:Variable
}

impl ScriptNodeList {
    pub fn new(fn_var:Variable) -> Self {
        ScriptNodeList { fn_var }
    }
}