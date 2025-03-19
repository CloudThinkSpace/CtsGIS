use axum::{Router, routing::post};

use crate::handler::user::add_user;

pub fn user_router() -> Router {
    Router::new().route("/", post(add_user))
}
