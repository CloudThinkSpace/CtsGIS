use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("server request param error: {0}")]
    RequestParamError(String),
    #[error("server error: {0}")]
    ResponseError(String),
}