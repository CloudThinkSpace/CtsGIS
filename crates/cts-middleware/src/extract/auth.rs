use axum::{
    Json,
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
    response::{IntoResponse, Response},
};
use cts_common::model::jwt::{AuthError, Claims, SECRET};
use jsonwebtoken::{DecodingKey, Validation};

pub struct AuthInfo(pub Claims);

/// > 定义用户信息提取器，提取用户编号和角色信息
///
impl<S> FromRequestParts<S> for AuthInfo
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or(
                (
                    StatusCode::UNAUTHORIZED,
                    Json(AuthError {
                        message: "Missing authorization header".to_string(),
                    }),
                )
                    .into_response(),
            )?;
        // 验证 JWT
        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        );
        // 权限
        let claims = claims.map(|data| data.claims).map_err(|_err| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    message: "Invalid token format".to_string(),
                }),
            )
                .into_response()
        })?;
        let auth_info = AuthInfo(claims);
        Ok(auth_info)
    }
}
