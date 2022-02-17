use std::{path::{PathBuf, Path}, fs, sync::Arc};
use lite_clojure_eval::EvalRT;
use seija_render::material::{MaterialDef,read_material_def};
use glsl_pkg::PackageManager;
use crate::backend::SeijaShaderBackend;
use anyhow::{Result, bail};
mod backend;

pub struct MaterialCompiler {
   folders:Vec<String>,
   pkg_mgr:PackageManager,
   backend:SeijaShaderBackend
}


impl MaterialCompiler {
   pub fn new() -> Self {
      MaterialCompiler { folders: vec![], pkg_mgr: PackageManager::new(),backend:SeijaShaderBackend::new() }
   }

   pub fn add_shader_dir(&mut self,dir:&str) {
      self.pkg_mgr.add_dir(dir)
   }

   pub fn set_shader_out(&mut self,dir:&str) {
      self.pkg_mgr.set_out_path(dir);
   }

   pub fn add_mat_search_path(&mut self,dir:&str) {
      self.folders.push(dir.to_string());
   }

   pub fn run(&mut self) {
      let mut tasks:Vec<ShaderTask> = vec![];
      for folder in self.folders.iter() {
         let mut search_str = String::from(folder);
         search_str.push_str("/**/*.mat.clj");
         for entry in glob::glob(&search_str).expect("Failed to read glob pattern") {
            match entry {
               Ok(path) => {
                  if let Err(err) = self.collect_mat_task(&path,&mut tasks) {
                     log::error!("compile material {:?} err:{:?}",path,err);
                  }
               },
               Err(e) => log::error!("read error {:?}",&e),
           }
         }
      }

      for task in tasks.iter() {
        if self.pkg_mgr.compile(&task.pkg_name, &task.shader_name, &task.macros, &self.backend) {
            log::info!("compile material success {}.{}",&task.pkg_name,&task.shader_name);
        }
      }
   }

   fn collect_mat_task(&self,path:&PathBuf,tasks:&mut Vec<ShaderTask>) -> Result<()> {
      let mut rt = EvalRT::new();
      let code = fs::read_to_string(path)?;
      let mat_def = read_material_def(&mut rt, &code)?;
      
      for pass in mat_def.pass_list.iter() {
         let names:Vec<_> = pass.shader_info.name.split('.').collect();
         if names.len() != 2 {
            bail!("shader name err:{}",pass.shader_info.name)
         }
         tasks.push(ShaderTask { 
            pkg_name: names[0].to_string(),
            shader_name: names[1].to_string(), 
            macros: pass.shader_info.macros.clone()  
         } );
      }

      Ok(())
   }
}

#[derive(Debug)]
struct ShaderTask {
   pkg_name:String,
   shader_name:String,
   macros:Arc<Vec<String>>
}

#[test]
fn test_fn() {
   let mut builder = env_logger::builder();
   builder.filter_level(log::LevelFilter::Info);
   builder.init();


   let mut mc = MaterialCompiler::new();
   mc.add_shader_dir("../shaders");
   mc.set_shader_out("../.shader_out");
   mc.add_mat_search_path("../../seija-examples/res/new_material");
   mc.run();
}