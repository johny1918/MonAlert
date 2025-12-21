use std::fs;

use crate::types::Config;
use crate::errors::CustomError;
use crate::config::validation::ConfigValidator;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_from_file(path: &str) -> Result<Config, CustomError> {
        let config_str = fs::read_to_string(path)
            .map_err(|e| CustomError::Error(format!("Failed to read file '{}': {}", path, e)))?;

        let config: Config = toml::from_str(&config_str)
            .map_err(|e| CustomError::Error(format!("Failed to parse TOML: {}", e)))?;

        // Validate the configuration
        ConfigValidator::validate(&config)?;

        Ok(config)
    }

    pub fn load_from_str(content: &str) -> Result<Config, CustomError> {
        let config: Config = toml::from_str(content)
            .map_err(|e| CustomError::Error(format!("Failed to parse TOML: {}", e)))?;

        // Validate the configuration
        ConfigValidator::validate(&config)?;

        Ok(config)
    }
}