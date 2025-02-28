use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
}

impl ServerConfig {
    pub fn listen_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}