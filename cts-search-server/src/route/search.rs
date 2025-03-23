use axum::Router;
use axum::routing::post;
use crate::handler::search::{query_handler, search_handler};

pub fn search_router() -> Router {
    Router::new()
        .route("/{table}", post(search_handler))
        .route("/{table}/{id}", post(query_handler))
}
