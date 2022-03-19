use std::path::PathBuf;
use std::fs;
use std::time::SystemTime;
use material_compiler::MaterialCompiler;

fn main() {
    let time_path = PathBuf::from(".render/lasttime.txt");
    let last_shader_time = read_shader_last_time();
   
    let str_time = if time_path.exists() {
        fs::read_to_string(&time_path).unwrap()
    } else {
        String::default()
    }; 
    if last_shader_time != str_time { 
        let mut mc = MaterialCompiler::new();
        mc.add_shader_dir("../crates/shaders");
        mc.set_shader_out(".render/shaders/");
        mc.add_mat_search_path("res/new_material");
        mc.run(".render/render.clj");
        fs::write(&time_path, last_shader_time).unwrap();
    }
}

fn read_shader_last_time() -> String {
    let path_buf = PathBuf::from("../crates/shaders");
    if let Ok(sys_time) = path_buf.metadata().and_then(|t| t.modified()) {
        let dt = sys_time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        return dt.as_millis().to_string();
    }
    String::default()
} 