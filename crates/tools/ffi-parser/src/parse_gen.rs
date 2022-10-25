use anyhow::{Result};
use serde_derive::Deserialize;

#[derive(Debug,Deserialize)]
pub struct ParseGenConfig {
    target:String,
    #[serde(rename = "headerDir")]
    header_dir:Option<String>,
    #[serde(rename = "outDir")]
    out_dir:Option<String>
}

impl ParseGenConfig {
    pub fn load(path:&str) -> Result<ParseGenConfig> {
        let config_code =  std::fs::read_to_string(path)?;
        let config:ParseGenConfig = toml::from_str(config_code.as_str())?;
        Ok(config)
    }
}

#[test]
fn test_load_config() {
    let config = ParseGenConfig::load("config.ini").unwrap();
    dbg!(config);
}


pub struct ParseGen {

}

impl ParseGen {
    pub fn run(&mut self,config:&ParseGenConfig) {
        
    }
}