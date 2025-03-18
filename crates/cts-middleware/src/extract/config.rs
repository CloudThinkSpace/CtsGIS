#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use cts_common::config::Config;

pub struct CtsConfig(pub Arc<Config>);
/// > 自定义提取器，提取数据库连接池对象
///
impl<S> FromRequestParts<S> for CtsConfig
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = parts
            .extensions
            .get::<Arc<Config>>()
            .ok_or((StatusCode::SERVICE_UNAVAILABLE, "无法提交config"))?;
        Ok(CtsConfig(config.clone()))
    }
}
