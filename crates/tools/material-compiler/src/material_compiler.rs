use std::{path::PathBuf, fs, sync::Arc};

use anyhow::{Result, bail,anyhow};
use glsl_pack_rtbase::shader::Shader;
use glsl_pkg::PackageManager;
use lite_clojure_eval::EvalRT;
use seija_render::material::{read_material_def, PassDef};

use crate::backend::SeijaShaderBackend;

pub struct CompilerConfig {
    pub material_paths:Vec<String>,
    pub script_path:String,
    pub script_libs:Vec<String>,
    pub shader_libs:Vec<String>,
}

impl CompilerConfig {
    pub fn new(script_path:&str) -> Self {
        CompilerConfig { 
            material_paths: vec![],
            script_path:script_path.into(),
            script_libs:vec![],
            shader_libs:vec![]
        }
    }

    pub fn add_material_path(&mut self,dir:&str) {
        self.material_paths.push(dir.to_string());
    }

    pub fn add_script_lib(&mut self,path:&str) {
        self.script_libs.push(path.into());
    }

    pub fn add_shader_lib_path(&mut self,dir:&str) {
        self.shader_libs.push(dir.into())
    }

}

pub struct MaterialCompiler {
    pkg_mgr:PackageManager,
    backend:SeijaShaderBackend,
}



impl MaterialCompiler {
    pub fn new() -> Self {
        MaterialCompiler { 
            pkg_mgr: PackageManager::new(),
            backend:SeijaShaderBackend::new(),
        }
    }

   
    

   

    pub fn run(&mut self,config:&CompilerConfig) {
        self.backend.init(&config.script_path);
        for material_path in config.material_paths.iter() {
            let search_str = format!("{}/**/*.mat.clj",material_path);
            let glob = glob::glob(&search_str).expect("Failed to read glob pattern");
            for entry in glob {
                match entry {
                    Ok(path) => {
                       if let Err(err) = self.process_material(&path) {
                           log::error!("material error path:{:?} err:{}",path,err);
                       }
                    },
                    Err(e) => log::error!("read error {:?}",&e),
                }
            }
        }
    }

    fn process_material(&mut self,path:&PathBuf) -> Result<()> {
        let mut rt = EvalRT::new();
        let mat_def = read_material_def(&mut rt, &fs::read_to_string(path)?)?;
        for pass_def in mat_def.pass_list.iter() {
            self.process_material_pass(pass_def)?
        }
        Ok(())
    }

    fn process_material_pass(&mut self,pass_def:&PassDef) -> Result<()> {
        let names:Vec<_> = pass_def.shader_info.name.split('.').collect();
        if names.len() != 2 { bail!("shader name err:{}",pass_def.shader_info.name) }
        let package = self.pkg_mgr.get_or_load_pkg(names[0])
                                                .ok_or(anyhow!("not found shader package {}",names[0]))?;
        let shader:&Arc<Shader> = package.info.find_shader(&names[1])
                                 .ok_or(anyhow!("not found shader in package {}.{}",names[0],names[1]))?;
        
        Ok(())
    }

    
}