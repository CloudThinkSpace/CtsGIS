use crate::handler::feature::feature_handler;
use axum::Router;
use axum::routing::post;

pub fn feature_router() -> Router {
    Router::new().route("/{feature}", post(feature_handler))
}
