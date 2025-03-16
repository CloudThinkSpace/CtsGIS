use axum::extract::DefaultBodyLimit;
use axum::routing::get;
use axum::{Router, middleware};
use cts_middleware::extract::auth::AuthInfo;
use cts_middleware::middleware::auth::auth_middleware;
use cts_middleware::middleware::log::logging_middleware;
use tower_http::cors::{Any, CorsLayer};

mod aggregate;
pub mod feature;
pub mod mbtiles;
mod search;

/// @description 路由整合函数
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
        .layer(middleware::from_fn(logging_middleware))
        .layer(middleware::from_fn(auth_middleware))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20))
}

async fn root(AuthInfo(_user): AuthInfo) -> &'static str {
    "Welcome to CTS GIS Server!"
}
