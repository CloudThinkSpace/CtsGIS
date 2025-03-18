use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
}
