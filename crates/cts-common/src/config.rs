use crate::config::database::Database;
use crate::config::error::ConfigError;
use crate::config::redis::RedisConfig;
use ::log::info;
use serde::{Deserialize, Serialize};
use std::fs;

pub mod database;
pub mod error;
pub mod log;
pub mod mbtiles;
pub mod oss;
pub mod redis;
pub mod server;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub database: Database,
    pub redis: Option<RedisConfig>,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        Self::load_with_config("config/config.yml")
    }

    pub fn load_with_config(path: &str) -> Result<Self, ConfigError> {
        info!("Read the configuration file from: {}", path);
        // load config file
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        info!("Loading the config file succeeded.");
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_config() {
        let _config = Config::load_with_config("config/config.yml");
        println!("{:?}", _config);
    }

    #[test]
    fn test_error() {}
}
