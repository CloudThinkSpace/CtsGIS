use crate::error::server::ServerError;
use thiserror::Error;

pub mod server;

#[derive(Debug, Error)]
pub enum CtsError {
    #[error("server error: {0}")]
    ServerError(#[from] ServerError),
}
