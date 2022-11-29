use clap::{Arg};
mod gltf2template;
mod scheme;

fn main() {
    let mut builder = env_logger::builder();
    builder.filter_level(log::LevelFilter::Info);
    builder.init();

   
}
