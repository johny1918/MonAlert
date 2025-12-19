use crate::types::Config;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_from_file(path: &str) -> Result<Config, std::io::Error> {
        
    }

    pub fn load_from_str(content: &str) -> Result<Config, std::io::Error> {

    }
}