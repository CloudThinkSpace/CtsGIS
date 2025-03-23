#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Request, middleware::Next};
use chrono::Local;
use cts_common::model::jwt::{AuthError, Claims};

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

#[cfg(test)]
mod test {

    use chrono::Local;
    use cts_common::model::jwt::{Claims, SECRET};
    use jsonwebtoken::{EncodingKey, Header};

    #[test]
    fn test1() {
        let aa = Local::now().timestamp_millis() + 3600000;
        let claims = Claims {
            sub: "123".to_string(),
            role: "123".to_string(),
            exp: aa as usize,
        };
        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET.as_ref()),
        );

        println!("{:?}", token);
    }
}
