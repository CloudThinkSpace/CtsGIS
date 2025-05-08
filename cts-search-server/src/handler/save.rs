use std::collections::HashMap;

use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use cts_middleware::extract::config::CtsConfig;
use cts_middleware::extract::db::DbPool;
use cts_sql_expression::expression::save_sql::SaveSqlBuilder;
use response_utils::res::ResResult;
use serde_json::Value;

pub async fn save_handler(
    DbPool(pool): DbPool,
    CtsConfig(config): CtsConfig,
    Path(table_name): Path<String>,
    Json(data): Json<HashMap<String, Value>>,
) -> impl IntoResponse {
    // 获取数据库连接池
    let pool = pool.as_ref();
    let schema = config.database.schema.clone();
    // 配置
    let result = SaveSqlBuilder::new(
        data,
        pool,
        table_name,
        schema.unwrap_or("public".to_string()),
    )
    .execute()
    .await;
    match result {
        Ok(data) => ResResult::with_success(data),
        Err(err) => ResResult::<()>::with_error(&err.to_string()),
    }
}
