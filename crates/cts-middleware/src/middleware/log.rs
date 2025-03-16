#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::time::Instant;
use tracing::info;

/// 日志打印中间件
/// @description 打印请求耗时
pub async fn logging_middleware(request: Request, next: Next) -> Response {
    // 请求方法
    let method = request.method().clone();
    // 请求路径
    let path = request.uri().path().to_owned();
    // 开始时间
    let start = Instant::now();
    // 打印请求信息
    info!("→→→ {} {} started", method, path);

    let response = next.run(request).await;

    let duration = start.elapsed();
    // 打印请求完成时间
    info!(
        "←←← {} {} completed in {}ms (status: {})",
        method,
        path,
        duration.as_millis(),
        response.status()
    );

    response
}
