use crate::config::error::ConfigError;
use crate::config::redis::RedisConfig;
use ::log::info;
use database::DatabaseConfig;
use serde::{Deserialize, Serialize};
use server::ServerConfig;
use std::fs;
use crate::config::storage::StorageConfig;

pub mod database;
pub mod error;
pub mod log;
pub mod mbtiles;
pub mod redis;
pub mod server;
pub mod storage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub redis: Option<RedisConfig>,
    pub server: ServerConfig,
    pub storage: Option<StorageConfig>,
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
