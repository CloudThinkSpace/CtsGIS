use axum::Router;
use axum::routing::post;

use crate::handler::update::update_handler;

pub fn update_router() -> Router {
    Router::new()
        .route("/{table}/{id}", post(update_handler))
}
