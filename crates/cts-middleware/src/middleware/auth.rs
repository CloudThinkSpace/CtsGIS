use axum::{
    Json,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::Local;
use cts_common::model::jwt::{AuthError, Claims};
use tracing::info;

use crate::extract::auth::AuthInfo;

/// @description token校验中间件
pub async fn auth_middleware(AuthInfo(claims): AuthInfo, request: Request, next: Next) -> Response {
    let result = verify(claims).await;
    match result {
        Ok(_) => next.run(request).await,
        Err(data) => data,
    }
}

/// 校验token是否有效
/// @param token 解析出来的token数据
/// return authInfo
pub async fn verify(claims: Claims) -> Result<(), Response> {
    // 判断token是否有效
    // 请求开始时间
    let start_time = Local::now().timestamp_millis() as usize;
    // token过期时间
    let exp_time = claims.exp;
    // 判断是否token时间是否过去
    if start_time > exp_time {
        info!("开始时间：{},\n结束时间：{}", start_time, exp_time);
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                message: "Invalid token format".to_string(),
            }),
        )
            .into_response());
    }
    Ok(())
}
