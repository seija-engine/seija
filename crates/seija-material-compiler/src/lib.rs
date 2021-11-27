pub struct CompilerEnv {
    shader_folders:Vec<String>
}


pub struct MaterialCompiler {

}


#[test]
fn test_shaderc() {
    use shaderc::*;
    let compile = Compiler::new().unwrap();
    let opts = CompileOptions::new().unwrap();
    
    
    
}