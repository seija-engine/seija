use std::{path::{PathBuf, Path}, fs, sync::Arc};

use anyhow::{Result, bail,anyhow};
use glsl_pack_rtbase::shader::Shader;
use glsl_pkg::PackageManager;
use lite_clojure_eval::EvalRT;
use seija_render::material::{read_material_def, PassDef, MaterialDef};
use serde::{Deserialize};

use crate::backend::{SeijaShaderBackend, ShaderTask};
#[derive(Default,Deserialize)]
pub struct CompilerConfig {
    pub material_paths:Vec<String>,
    pub script_path:String,
    pub out_path:String,
    pub script_libs:Vec<String>,
    pub shader_libs:Vec<String>,
}

impl CompilerConfig {
    pub fn new(script_path:&str) -> Self {
        CompilerConfig { 
            out_path:String::default(),
            material_paths: vec![],
            script_path:script_path.into(),
            script_libs:vec![],
            shader_libs:vec![]
        }
    }

    pub fn form_json<P:AsRef<Path>>(path:P) -> Result<CompilerConfig> {
        let de:CompilerConfig = serde_json::from_str(std::fs::read_to_string(path.as_ref())?.as_str() )?;
        Ok(de)
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

    pub fn set_shader_out(&mut self,dir:&str) {
         self.out_path = dir.into();
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
        for script_path in config.script_libs.iter() {
            self.backend.render_info.rsc.rt.add_search_path(script_path);
        }
        for shader_path in config.shader_libs.iter() {
            self.pkg_mgr.add_dir(shader_path);
        }
        self.pkg_mgr.set_out_path(&config.out_path);
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
        self.pkg_mgr.write_rtinfos();
    }

    fn process_material(&mut self,path:&PathBuf) -> Result<()> {
        let mut rt = EvalRT::new();
        let mat_def = read_material_def(&mut rt, &fs::read_to_string(path)?)?;
        for pass_def in mat_def.pass_list.iter() {
            self.process_material_pass(pass_def,&mat_def)?
        }
        Ok(())
    }

    fn process_material_pass(&mut self,pass_def:&PassDef,material_def:&MaterialDef) -> Result<()> {
        let names:Vec<_> = pass_def.shader_info.name.split('.').collect();
        if names.len() != 2 { bail!("shader name err:{}",pass_def.shader_info.name) }
        let package = self.pkg_mgr.get_or_load_pkg(names[0])
                                                .ok_or(anyhow!("not found shader package {}",names[0]))?;
        let shader:&Arc<Shader> = package.info.find_shader(&names[1])
                                              .ok_or(anyhow!("not found shader in package {}.{}",names[0],names[1]))?;
        let macros = shader.get_macros(&pass_def.shader_info.features);
        let backends = shader.get_backends(&pass_def.shader_info.features);
        let shader_task = ShaderTask {
            pkg_name: names[0].to_string(),
            shader_name: names[1].to_string(), 
            macros: Arc::new(macros),
            prop_def:material_def.prop_def.clone(),
            tex_prop_def:material_def.tex_prop_def.clone(),
            slots:pass_def.shader_info.slots.clone(),
            backends
        };
        if self.pkg_mgr.compile(&shader_task.pkg_name, &shader_task.shader_name, &shader_task.macros, &self.backend,&shader_task) {
            log::info!("compile material success {}.{}",&shader_task.pkg_name,&shader_task.shader_name);
        }

        Ok(())
    }

    
}