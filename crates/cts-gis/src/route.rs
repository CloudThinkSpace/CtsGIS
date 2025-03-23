use auth::auth_router;
use axum::extract::DefaultBodyLimit;
use axum::{Router, middleware};
use base::noauth_router;
use cts_middleware::middleware::log::logging_middleware;
use tower_http::cors::{Any, CorsLayer};

pub mod auth;
pub mod base;

/// @description 路由整合函数
pub fn root_route() -> Router {
    // 跨域处理层
    let cors = CorsLayer::new().allow_methods(Any).allow_headers(Any);

    Router::new()
        .merge(noauth_router())
        .merge(auth_router())
        .layer(cors)
        .layer(middleware::from_fn(logging_middleware))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20))
}
