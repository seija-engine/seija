use crate::backend::SeijaShaderBackend;

mod backend;


#[test]
fn test_fn() {
    use glsl_pkg::{CompileConfig,CompileTask,Compiler,CompileEnv};
    let mut cfg = CompileConfig::new(SeijaShaderBackend::new());
    cfg.set_source_path("path");
    let mut compiler = Compiler::new(cfg);
    let task = CompileTask::new("name", vec![]);
    compiler.run_task(&task);
}