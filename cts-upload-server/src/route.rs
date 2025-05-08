use crate::upload_download::{download, image, upload, upload_table};
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::{middleware, Router};
use cts_middleware::middleware::log::logging_middleware;
use tower_http::cors::{Any, CorsLayer};

/// @description 路由整合函数1
pub fn root_route() -> Router {
    // 跨域处理层
    let cors = CorsLayer::new().allow_methods(Any).allow_headers(Any);

    Router::new()
        .route("/upload", post(upload))
        .route("/upload/{table}", post(upload_table))
        .route("/browse/{*path}", get(image))
        .route("/download/{*path}", get(download))
        .layer(cors)
        .layer(middleware::from_fn(logging_middleware))
        // 上传文件最大1G
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
}
