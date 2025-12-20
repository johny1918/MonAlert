use std::fs;

use crate::types::Config;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_from_file(path: &str) -> Result<Config, std::io::Error> {
        let config = fs::read_to_string(path).expect("Failed to find the file");
        let config_deserialized: Config = toml::from_str(&config).expect("Failed to read the config file");
        Ok(config_deserialized)
        
    }

    pub fn load_from_str(content: &str) -> Result<Config, std::io::Error> {
        todo!()
    }
}