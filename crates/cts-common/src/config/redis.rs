use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u32,
    pub password: Option<String>,
    pub database: Option<String>,
}
