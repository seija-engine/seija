mod backend;
mod render_info;
mod material_compiler;
pub use material_compiler::{MaterialCompiler,CompilerConfig};

#[test]
fn 
test_fn() {
   let mut builder = env_logger::builder();
   
   builder.filter_level(log::LevelFilter::Info);
   builder.init();

   let mut config = CompilerConfig::new("../../../seija-examples/.render/shadow_render.clj");
   config.add_shader_lib_path("../../shaders");
   config.add_script_lib("../../seija-pbr/res");
   config.add_script_lib("../../seija-render/res");
   config.add_material_path("../../../seija-examples/res/materials");
   config.set_shader_out("../../../seija-examples/.render/shaders");

   MaterialCompiler::new().run(&config);
}

