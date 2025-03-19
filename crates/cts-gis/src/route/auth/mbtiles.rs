use crate::handler::mbtiles::mbtiles_handler;
use axum::Router;
use axum::routing::post;

pub fn mbtiles_router() -> Router {
    Router::new().route("/{tiles}", post(mbtiles_handler))
}
