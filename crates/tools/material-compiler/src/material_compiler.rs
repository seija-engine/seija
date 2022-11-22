use std::{path::{PathBuf, Path}, fs, sync::Arc, collections::{HashMap, hash_map::DefaultHasher, HashSet}};
use std::hash::{Hash,Hasher};
use anyhow::{Result, bail,anyhow};
use glsl_pack_rtbase::shader::Shader;
use glsl_pkg::PackageManager;
use lite_clojure_eval::EvalRT;
use seija_render::material::{read_material_def, PassDef, MaterialDef};
use serde_derive::Deserialize;

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
    compiled:HashSet<u64>,
    cache_shaders:HashSet<u64>
}



impl MaterialCompiler {
    pub fn new() -> Self {
        MaterialCompiler { 
            pkg_mgr: PackageManager::new(),
            backend:SeijaShaderBackend::new(),
            compiled:HashSet::default(),
            cache_shaders:HashSet::default()
        }
    }

    pub fn run(&mut self,config:&CompilerConfig) {
        let cache_path = format!("{}/cahce.json",config.out_path);
        if let Ok(code_string) = std::fs::read_to_string(&cache_path) {
            self.cache_shaders = serde_json::de::from_str(code_string.as_str()).unwrap();
        }
        self.compiled.clear();
        for script_path in config.script_libs.iter() {
            self.backend.render_info.system.vm.add_search_path(script_path);
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
        let ser_string = serde_json::ser::to_string(&self.cache_shaders).unwrap();
        std::fs::write(&cache_path, &ser_string).unwrap();
    }

    fn process_material(&mut self,path:&PathBuf) -> Result<()> {
        let mut rt = EvalRT::new();
        let mat_def = read_material_def(&mut rt, &fs::read_to_string(path)?,true)?;
        for (index,pass_def) in mat_def.pass_list.iter().enumerate() {
            self.process_material_pass(pass_def,&mat_def,index)?
        }
        Ok(())
    }

    fn process_material_pass(&mut self,pass_def:&PassDef,material_def:&MaterialDef,index:usize) -> Result<()> {
        let names:Vec<_> = pass_def.shader_info.name.split('.').collect();
        if names.len() != 2 { bail!("shader name err:{}",pass_def.shader_info.name) }
        let package = self.pkg_mgr.get_or_load_pkg(names[0])
                                                .ok_or(anyhow!("not found shader package {}",names[0]))?;
        let shader:&Arc<Shader> = package.info.find_shader(&names[1])
                                              .ok_or(anyhow!("not found shader in package {}.{}",names[0],names[1]))?;
        let mut macros = shader.get_macros(&pass_def.shader_info.features);
        let backends = shader.get_backends(&pass_def.shader_info.features);
        if let Some(slot_string) = pass_def.shader_info.slots.as_ref() {
            let mut hasher = DefaultHasher::default();
            slot_string.hash(&mut hasher);
            let hash_code = hasher.finish().to_string();
            macros.push(hash_code.into());
        }
        let shader_task = ShaderTask {
            pkg_name: names[0].to_string(),
            shader_name: names[1].to_string(), 
            macros: Arc::new(macros),
            prop_def:material_def.prop_def.clone(),
            tex_prop_def:material_def.tex_prop_def.clone(),
            slots: pass_def.shader_info.slots.as_ref().map(|v|Self::process_slot_string(v.as_str())).unwrap_or(HashMap::default()),
            backends
        };
        let task_hash = shader_task.hash_code();
        if self.compiled.contains(&task_hash) {
            log::info!("skip {}.{}",&material_def.name,index);
            return Ok(());
        }
       
        if self.pkg_mgr.compile(&shader_task.pkg_name, 
                                &shader_task.shader_name, 
                                &shader_task.macros, 
                      &mut self.cache_shaders,
                               &self.backend,&shader_task) {
            log::info!("compile material success {}.{}",&shader_task.pkg_name,&shader_task.shader_name);
        }
        self.compiled.insert(shader_task.hash_code());
        Ok(())
    }

    fn process_slot_string(string:&str) -> HashMap<String,String> {
        let mut glsl_func = GLSLFuncReader::default();
        let mut slot_dict = HashMap::default();
        glsl_func.reset();
        for chr in string.chars() {
           if glsl_func.on_char(chr) {
              slot_dict.insert(glsl_func.fn_name.take().unwrap(), glsl_func.body.take().unwrap());
              glsl_func.reset();
           }
        }
        slot_dict
    }

    
}

#[derive(Debug)]
enum GLSLFuncState { None,ReadType, ReadTypeEnd, ReadName, ReadBody }
impl Default for GLSLFuncState {
    fn default() -> Self {  GLSLFuncState::None }
}

#[derive(Default,Debug)]
struct GLSLFuncReader {
    fn_name:Option<String>,
    ret_type:Option<String>,
    body:Option<String>,
    state:GLSLFuncState,
    wait_body_end:bool,
    wait_count:u32
}

impl GLSLFuncReader {
    fn reset(&mut self) {
        self.wait_body_end = false;
        self.state = GLSLFuncState::None;
        self.fn_name = Some(String::default() );
        self.ret_type = Some(String::default() );
        self.body = Some(String::default() );
        self.wait_count = 0u32;
    }

    fn on_char(&mut self,chr:char) -> bool {
        match self.state {
            GLSLFuncState::None => {
                if !chr.is_whitespace() {
                    self.body.as_mut().map(|s| s.push(chr));
                    self.ret_type.as_mut().map(|s| s.push(chr));
                    self.state = GLSLFuncState::ReadType; 
                }
            },
            GLSLFuncState::ReadType => {
                self.body.as_mut().map(|s| s.push(chr));
                if chr.is_whitespace() {
                    self.state = GLSLFuncState::ReadTypeEnd
                } else {
                    self.ret_type.as_mut().map(|s| s.push(chr));
                }
            },
            GLSLFuncState::ReadTypeEnd => {
                self.body.as_mut().map(|s| s.push(chr));
                if !chr.is_whitespace() {
                    self.fn_name.as_mut().map(|s| s.push(chr));
                    self.state = GLSLFuncState::ReadName 
                }
            },
            GLSLFuncState::ReadName => {
                self.body.as_mut().map(|s| s.push(chr));
                if chr == '(' {
                    self.state = GLSLFuncState::ReadBody
                } else {
                    self.fn_name.as_mut().map(|s| s.push(chr));
                }
            }
            GLSLFuncState::ReadBody => {
                self.body.as_mut().map(|s| s.push(chr));
                if chr == '{' {
                    if !self.wait_body_end {
                        self.wait_body_end = true;
                    }
                    self.wait_count += 1;
                }
                if chr == '}' {
                    self.wait_count -= 1;
                }
                if self.wait_body_end && self.wait_count == 0 {
                    return true;
                }
            }
        }
        false
    }
}