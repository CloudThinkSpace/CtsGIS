use axum::{Router, routing::post};

use crate::handler::base::login;

pub fn base_router() -> Router {
    Router::new().route("/login", post(login))
}
