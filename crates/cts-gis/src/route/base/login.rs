use crate::handler::base::{login, logout};
use axum::{Router, routing::get, routing::post};

pub fn login_router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
}
