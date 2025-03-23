use axum::extract::DefaultBodyLimit;
use axum::{Router, middleware};
use cts_middleware::middleware::log::logging_middleware;
use tower_http::cors::{Any, CorsLayer};
use crate::route::search::search_router;

mod search;

/// @description 路由整合函数
pub fn root_route() -> Router {
    // 跨域处理层
    let cors = CorsLayer::new().allow_methods(Any).allow_headers(Any);

    Router::new()
        .merge(search_router())
        .layer(cors)
        .layer(middleware::from_fn(logging_middleware))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20))
}
