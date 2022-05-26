use std::{path::{PathBuf}, fs, sync::Arc, collections::HashMap};
use lite_clojure_eval::EvalRT;
use seija_render::{material::{read_material_def, TexturePropDef}, UniformBufferDef};
use glsl_pkg::PackageManager;
use crate::backend::SeijaShaderBackend;
use anyhow::{Result, bail};
mod backend;
mod render_info;

pub struct MaterialCompiler {
   folders:Vec<String>,
   pkg_mgr:PackageManager,
   pub backend:SeijaShaderBackend
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

   pub fn run(&mut self,config_path:&str) {
      self.backend.init(config_path);
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
        if self.pkg_mgr.compile(&task.pkg_name, &task.shader_name, &task.macros, &self.backend,&task) {
            log::info!("compile material success {}.{}",&task.pkg_name,&task.shader_name);
        }
      }
      self.pkg_mgr.write_rtinfos();
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
            macros: pass.shader_info.macros.clone(),
            prop_def:mat_def.prop_def.clone(),
            tex_prop_def:mat_def.tex_prop_def.clone(),
            slots:pass.shader_info.slots.clone()
         } );
      }

      Ok(())
   }
}

#[derive(Debug)]
pub struct ShaderTask {
   pkg_name:String,
   shader_name:String,
   macros:Arc<Vec<String>>,
   pub prop_def:Arc<UniformBufferDef>,
   pub tex_prop_def:Arc<TexturePropDef>,
   pub slots:HashMap<String,String>
}

#[test]
fn test_fn() {
   let mut builder = env_logger::builder();
   builder.filter_level(log::LevelFilter::Info);
   builder.init();

   
   let mut mc = MaterialCompiler::new();
   mc.add_shader_dir("../../shaders");
   mc.backend.render_info.rsc.rt.add_search_path("../../seija-pbr/res");
   mc.backend.render_info.rsc.rt.add_search_path("../../seija-render/res");
   mc.set_shader_out("../../../seija-examples/.render/shaders");
   mc.add_mat_search_path("../../../seija-examples/res/materials");
   mc.run("../../../seija-examples/.render/render.clj");

}

