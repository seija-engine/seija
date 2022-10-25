use std::{fs::DirEntry, path::{Path, PathBuf}};

use anyhow::{Result};
use serde_derive::Deserialize;

use crate::{parser::FFIFileParser, IGenerator, ffi_file::FFIFile, csharp_gen::CSharpGen};

#[derive(Debug,Deserialize)]
pub struct ParseGenConfig {
    pub target:String,
    #[serde(rename = "headerDir")]
    pub header_dir:String,
    #[serde(rename = "outDir")]
    pub out_dir:String,
    #[serde(rename = "dllName")]
    pub dll_name:String
}

impl ParseGenConfig {
    pub fn load(path:&str) -> Result<ParseGenConfig> {
        let config_code =  std::fs::read_to_string(path)?;
        let config:ParseGenConfig = toml::from_str(config_code.as_str())?;
        Ok(config)
    }
}



pub struct ParseGen {
    files:Vec<FFIFile>,
    config:ParseGenConfig
}

impl ParseGen {
    pub fn new(config:ParseGenConfig) -> Self {
        ParseGen { files: vec![], config }
    }

    pub fn load_files(&mut self) -> Result<()> {
       let dir_info = std::fs::read_dir(&self.config.header_dir)?;
       for item in dir_info {
           if let Ok(item) = item {
                if item.path().is_file() && item.path().to_string_lossy().ends_with(".h") {
                    let mut new_path = item.path();
                    new_path.set_extension("");
                    let  file_name:String = new_path.file_name().unwrap().to_string_lossy().into();
                
                    let code_string = std::fs::read_to_string(item.path())?;
                    let mut parser = FFIFileParser::new(&code_string,file_name);
                    match parser.parse() {
                        Ok(ffi_file) => {
                            self.files.push(ffi_file);
                        },
                        Err(err) => {
                            eprintln!("parse {:?} error:{:?}",item.path(),err);
                        }
                    }
                }
           }
       }
       Ok(())
    }

    pub fn run(&mut self,generator:impl IGenerator) -> Result<()> {
        //let dir_info = std::fs::d(&self.config.out_dir)?;
       let out_path = PathBuf::from(&self.config.out_dir);
       if !out_path.exists() {
          std::fs::create_dir_all(&out_path).unwrap();
       }

        for ffi_file in self.files.iter() {
           match generator.on_process(ffi_file,&self.config) {
                Ok(code_string) => {
                   let mut file_path = out_path.clone();
                   file_path = file_path.join(format!("{}.cs",ffi_file.name.as_str()));
                   std::fs::write(&file_path, code_string)?;
                },
                Err(err) => {
                    eprintln!("gen file {} error:{}",ffi_file.name.as_str(),err);
                }
           }
        }

        Ok(())
    }


}

#[test]
fn test_run() {
    let config = ParseGenConfig::load("config.ini").unwrap();
    let mut parse_gen = ParseGen::new(config);
    parse_gen.load_files().unwrap();
    
    parse_gen.run(CSharpGen ).unwrap();
}
