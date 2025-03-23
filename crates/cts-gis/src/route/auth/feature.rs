use crate::handler::feature::{feature_handler, feature_id_handler};
use axum::Router;
use axum::routing::post;

pub fn feature_router() -> Router {
    Router::new()
        .route("/{feature}", post(feature_handler))
        .route("/{feature}/{id}", post(feature_id_handler))
}
