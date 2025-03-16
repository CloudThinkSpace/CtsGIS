#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use sqlx::PgPool;

pub struct DbPool(pub Arc<PgPool>);
/// > 自定义提取器，提取数据库连接池对象
///
impl<S> FromRequestParts<S> for DbPool
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = parts
            .extensions
            .get::<Arc<PgPool>>()
            .ok_or((StatusCode::SERVICE_UNAVAILABLE, "无法提交pool"))?;
        Ok(DbPool(pool.clone()))
    }
}
