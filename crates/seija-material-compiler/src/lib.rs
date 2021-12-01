use std::{path::{PathBuf, Path}};
use material_task::MaterialTask;
use shader_env::ShaderEnv;
use walkdir::WalkDir;
mod material_task;
mod shader_env;

#[derive(Default)]
pub struct CompilerConfig {
    shader_folders:Vec<PathBuf>,
    material_folders:Vec<PathBuf>,
   
}

impl CompilerConfig {
    pub fn add_shader_path<P:AsRef<Path>>(&mut self,path:P) {
        self.shader_folders.push(path.as_ref().to_path_buf());
    }

    pub fn add_material_path<P:AsRef<Path>>(&mut self,path:P) {
        self.material_folders.push(path.as_ref().to_path_buf());
    }
}


#[derive(Default)]
pub struct MaterialCompiler {
    shader_env:ShaderEnv
}

impl MaterialCompiler {
    pub fn start(&mut self,config:&CompilerConfig) {
        self.scan_shaders(config);

        for f in config.material_folders.iter() {
            for dir in WalkDir::new(f).into_iter().filter_map(Result::ok).filter(|f| f.file_type().is_file() ) {
                MaterialTask::default().run(dir.path());
            }
        }
    }

    fn scan_shaders(&mut self,config:&CompilerConfig) {
        for p in config.shader_folders.iter() {
            self.shader_env.scan(p);
        }
    }
}

#[test]
fn test_shaderc() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    
    let mut config = CompilerConfig::default();
    let cur_dir = std::env::current_dir().unwrap();
    let sp = cur_dir.join("../shaders");
    let mp = cur_dir.join("../../seija-examples/res/new_material");
    config.add_shader_path(sp.to_str().unwrap());
    config.add_material_path(mp.to_str().unwrap());
  

    let mut mc = MaterialCompiler::default();
    mc.start(&config);
}