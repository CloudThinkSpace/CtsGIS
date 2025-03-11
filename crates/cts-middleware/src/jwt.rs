#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Request, middleware::Next};
use cts_common::model::jwt::{AuthError, Claims, SECRET, UserContext};
use jsonwebtoken::{DecodingKey, Validation};

// 中间件实现
async fn auth_middleware(request: Request, next: Next) -> Response {
    // 提取 Token
    let (mut parts, body) = request.into_parts();
    // 手动获取 Authorization 头
    let auth_header = parts
        .headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let auth_header = match auth_header {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    message: "Missing authorization header".to_string(),
                }),
            )
                .into_response();
        }
    };
    // 解析 Bearer Token
    let token = auth_header.strip_prefix("Bearer ");
    let token = match token {
        Some(data) => data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    message: "Invalid token format".to_string(),
                }),
            )
                .into_response();
        }
    };

    // 验证 JWT
    let claims = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    );

    let claims = match claims {
        Ok(data) => data.claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    message: "Invalid token".to_string(),
                }),
            )
                .into_response();
        }
    };

    // 将用户信息注入请求
    parts.extensions.insert(UserContext {
        user_id: claims.sub,
        role: claims.role,
    });
    next.run(Request::from_parts(parts, body)).await
}
