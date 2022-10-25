mod ffi_file;
mod parser;
mod lex_string;
use anyhow::{Result};
mod parse_gen;
use ffi_file::FFIFile;
pub use parse_gen::ParseGen;
use parse_gen::ParseGenConfig;
mod csharp_gen;

pub trait IGenerator {
   fn on_process(&self,ffi_file:&FFIFile,config:&ParseGenConfig) -> Result<String>;
}