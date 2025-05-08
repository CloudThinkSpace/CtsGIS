use axum::Router;
use axum::routing::post;
use crate::handler::save::save_handler;

pub fn save_router() -> Router {
    Router::new()
        .route("/{table}", post(save_handler))
}
