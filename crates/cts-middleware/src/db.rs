#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use sqlx::PgPool;
pub struct DbPool(PgPool);
/// > 自定义提取器，提取数据库连接池对象
///
impl<S> FromRequestParts<S> for DbPool
where
    S: Send + Sync,
    PgPool: FromRef<S>,
{
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(_parts: &mut Parts, state: &S) -> impl Future<Output=Result<Self, Self::Rejection>> + Send {
        let pool = PgPool::from_ref(state);
        async { Ok(DbPool(pool)) }
    }
}