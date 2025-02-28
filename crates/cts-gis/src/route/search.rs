use crate::handler::search::search_handler;
use axum::Router;
use axum::routing::post;

pub fn search_router() -> Router {
    Router::new().route("/", post(search_handler))
}
