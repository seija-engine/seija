
use clap::{Parser};
use gltf2template::{ExportConfig, GlTF2Template};
mod gltf2template;
mod scheme;

#[derive(Debug,Parser)]
#[command(author, version, about, long_about = None)]
struct ARGS {
    path:String
}

fn main() {
    let mut builder = env_logger::builder();
    builder.filter_level(log::LevelFilter::Info);
    builder.init();
    let args = ARGS::parse();
    let opts = ExportConfig::default();
    let mut template = GlTF2Template::default();
    template.run(&args.path, &opts).unwrap();
}
