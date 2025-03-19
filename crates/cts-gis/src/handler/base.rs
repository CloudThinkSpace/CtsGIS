use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::Local;
use cts_common::model::jwt::SECRET;
use cts_common::model::{jwt::Claims, request::user::RequestUser};
use cts_middleware::extract::db::DbPool;
use jsonwebtoken::{EncodingKey, Header};
use response_utils::res::ResResult;

pub async fn login(DbPool(_pool): DbPool, Json(user): Json<RequestUser>) -> impl IntoResponse {
    let exp = Local::now().timestamp_millis() + 3600 * 1000;
    let claims = Claims {
        sub: user.username,
        role: "".to_string(),
        exp: exp as usize,
    };
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    );

    match token {
        Ok(data) => {
            println!("{:?}", &data);
            data.into_response()
        }
        Err(_) => (StatusCode::UNAUTHORIZED, "1111").into_response(),
    }
}

pub async fn logout(DbPool(_pool): DbPool, Json(_user): Json<RequestUser>) -> impl IntoResponse {
    ResResult::with_success("退出成功")
}
