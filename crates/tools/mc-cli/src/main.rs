use std::path::PathBuf;

use clap::{App,Arg};
use material_compiler::{CompilerConfig,MaterialCompiler};
fn main() {
    let mut builder = env_logger::builder();
    builder.filter_level(log::LevelFilter::Info);
    builder.init();

    let matchs = App::new("mc-cli")
                          .version("0.1.0")
                          .arg(Arg::new("config").short('c').default_value("config.json").required(false).long("config"))
                          .arg(Arg::new("out").short('o').required(false).long("out"))
                          .get_matches();

    let config_path = matchs.value_of("config").unwrap();
    let cur_path = std::env::current_dir().expect("current_dir");
    match CompilerConfig::form_json(cur_path.join(config_path) ) {
        Ok(mut config) => {
            let mut mc = MaterialCompiler::new();
            if let Some(out_path)  = matchs.value_of("out") {
                config.set_shader_out(out_path);
            }
            mc.run(&config);
        },
        Err(err) => {
            eprintln!("read config error:{:?} {:?}",cur_path.join(config_path),err);
        }
    }
   
    
}
