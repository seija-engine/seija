use std::path::{PathBuf, Path};

use glsl_pkg::PackageManager;
use crate::backend::SeijaShaderBackend;
mod backend;

pub struct MaterialCompiler {
   folders:Vec<PathBuf>,
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
      self.folders.push(PathBuf::from(dir));
   }

   pub fn run(&mut self) {

   }
}

#[test]
fn test_fn() {
   let backend = SeijaShaderBackend::new();
   let mut pkg_mgr = PackageManager::new();
   pkg_mgr.add_dir("../shaders");
   pkg_mgr.set_out_path("../.shader_out");
   pkg_mgr.compile("core", "color", vec![], &backend);
   
}