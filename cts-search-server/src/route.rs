use crate::route::aggregate::aggregate_router;
use crate::route::search::search_router;
use crate::route::r#static::static_data;
use axum::extract::DefaultBodyLimit;
use axum::routing::get;
use axum::{Router, middleware};
use cts_middleware::middleware::log::logging_middleware;
use delete::delete_router;
use save::save_router;
use tower_http::cors::{Any, CorsLayer};
use update::update_router;

mod aggregate;
mod delete;
mod save;
mod search;
pub mod r#static;
mod update;

/// @description 路由整合函数
pub fn root_route() -> Router {
    // 跨域处理层
    let cors = CorsLayer::new().allow_methods(Any).allow_headers(Any);

    Router::new()
        .route("/data/{*path}", get(static_data))
        .nest("/search", search_router())
        .nest("/save", save_router())
        .nest("/delete", delete_router())
        .nest("/update", update_router())
        .nest("/aggregate", aggregate_router())
        .layer(cors)
        .layer(middleware::from_fn(logging_middleware))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20))
}
