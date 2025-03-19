use crate::handler::aggregate::aggregate_handler;
use axum::Router;
use axum::routing::post;

pub fn aggregate_router() -> Router {
    Router::new().route("/{table_name}", post(aggregate_handler))
}
