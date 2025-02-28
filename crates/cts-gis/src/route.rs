use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::get;
use tower_http::cors::{Any, CorsLayer};

mod aggregate;
pub mod feature;
pub mod mbtiles;
mod search;

pub fn root_route() -> Router {
    // 跨域处理层
    let cors = CorsLayer::new().allow_methods(Any).allow_headers(Any);
    // 整合所有请求路径
    Router::new()
        .route("/", get(root))
        .nest("/feature", feature::feature_router())
        .nest("/mbtiles", mbtiles::mbtiles_router())
        .nest("/search", search::search_router())
        .nest("/aggregate", aggregate::aggregate_router())
        // .route("/webui", )
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20))
}

async fn root() -> &'static str {
    "Welcome to CTS GIS Server!"
}
