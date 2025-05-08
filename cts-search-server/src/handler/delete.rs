use axum::extract::Path;
use axum::response::IntoResponse;
use cts_middleware::extract::config::CtsConfig;
use cts_middleware::extract::db::DbPool;
use cts_sql_expression::expression::delete_sql::DeleteSqlBuilder;
use response_utils::res::ResResult;

pub async fn delete_handler(
    DbPool(pool): DbPool,
    CtsConfig(config): CtsConfig,
    Path((table_name, id)): Path<(String, String)>,
) -> impl IntoResponse {
    // 获取数据库连接池
    let pool = pool.as_ref();
    let schema = config.database.schema.clone();
    // 表达式配置
    let result = DeleteSqlBuilder::new(
        pool,
        table_name,
        schema.unwrap_or(String::from("public")),
        id,
    )
    .execute()
    .await;
    match result {
        Ok(data) => ResResult::with_success(data),
        Err(err) => ResResult::<()>::with_error(&err.to_string()),
    }
}
