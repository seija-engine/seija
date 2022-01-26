use glsl_pkg::PackageManager;

use crate::backend::SeijaShaderBackend;

mod backend;


#[test]
fn test_fn() {
   let backend = SeijaShaderBackend::new();
   let mut pkg_mgr = PackageManager::new();
   pkg_mgr.add_dir("../shaders");
   pkg_mgr.set_out_path("../.shader_out");
   pkg_mgr.compile("core", "color", vec![], &backend);
}