use crate::config::error::database::DatabaseError;
use thiserror::Error;

mod database;
mod file;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("config not found error: {0}")]
    ConfigNotFound(#[from] std::io::Error),
    #[error("database error: {0}")]
    DatabaseError(#[from] DatabaseError),
    #[error("parse config error: {0}")]
    ParseError(#[from] serde_yaml::Error),
}
