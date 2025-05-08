use axum::Router;
use axum::routing::post;
use crate::handler::delete::delete_handler;

pub fn delete_router() -> Router {
    Router::new()
        .route("/{table}/{id}", post(delete_handler))
}
