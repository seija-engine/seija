use std::path::{Path,PathBuf};
#[derive(Default)]
pub struct CompilerConfig {
    shader_folders:Vec<String>,
    material_folders:Vec<String>
}

impl CompilerConfig {
    pub fn add_shader_path(&mut self,path:&str) {
        self.shader_folders.push(path.to_string());
    }

    pub fn add_material_path(&mut self,path:&str) {
        self.material_folders.push(path.to_string());
    }
}


pub struct MaterialCompiler {

}

impl MaterialCompiler {
    pub fn start(&self,env:&CompilerConfig) {
        for f in env.material_folders.iter() {
            self.compile_material_folder(f.as_str());
        }
    }

    fn compile_material_folder(&self,folder:&str) {
        
    }
}

#[test]
fn test_shaderc() {
    let mut config = CompilerConfig::default();
    let cur_dir = std::env::current_dir().unwrap();
    let sp = cur_dir.join("../shaders");
    config.add_shader_path(sp.to_str().unwrap());
    println!("cur dir:{:?}",cur_dir);
}