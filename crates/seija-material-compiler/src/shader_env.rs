use std::path::Path;
use serde_json::{Value};
use lite_clojure_eval::EvalRT;

const  SHADER_PACKAGE_PATH:&str = "package.clj";

#[derive(Default)]
pub struct ShaderEnv {

}

impl ShaderEnv {
    pub fn scan<P:AsRef<Path>>(&mut self,p:P) {
        let shader_list_path = p.as_ref().join(SHADER_PACKAGE_PATH);
        let all_string = std::fs::read_to_string(&shader_list_path);
        if all_string.is_err() {
            log::error!("not found {:?}",shader_list_path);
            return;
        }
        let mut eval_rt = EvalRT::new();
        eval_rt.init();
        let ret:Option<Value> = eval_rt.eval_string("".to_string(), all_string.as_ref().unwrap())
                                                .map(|v| v.into());
        if ret.is_none() {
            log::error!("package.clj error");
            return;
        }
        let json = ret.unwrap();
        
        let zero_vec = vec![];
        let shaders = json.get(":shaders").and_then(|v| v.as_array()).unwrap_or(&zero_vec);
        for shader_json in shaders {
            dbg!(&shader_json);
        }
    }
}