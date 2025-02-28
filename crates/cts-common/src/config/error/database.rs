use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Timeout error")]
    Timeout,
    #[error("user not found error")]
    UserNotFound,
    #[error("password error")]
    PasswordError,
}
