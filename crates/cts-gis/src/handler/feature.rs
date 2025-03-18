use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use cts_middleware::extract::db::DbPool;
use cts_sql_expression::config::ExpressionConfig;
use cts_sql_expression::expression::sql::SqlBuilder;
use cts_sql_expression::request::CtsParam;
use response_utils::res::ResResult;

pub async fn feature_handler(
    DbPool(pool): DbPool,
    Path(feature_name): Path<String>,
    Json(param): Json<CtsParam>,
) -> impl IntoResponse {
    // 获取数据库连接池
    let pool = pool.as_ref();
    // 配置
    let config = ExpressionConfig::new(None);
    let result = SqlBuilder::new(pool, feature_name, config, param)
        .query()
        .await;
    match result {
        Ok(data) => {
            let result = data.to_json();
            ResResult::with_success(result)
        }
        Err(err) => ResResult::<()>::with_error(&err.to_string()),
    }
}

pub async fn feature_id_handler(
    DbPool(pool): DbPool,
    Path((feature_name, id)): Path<(String, String)>,
    Json(param): Json<CtsParam>,
) -> impl IntoResponse {
    // 获取数据库连接池
    let pool = pool.as_ref();
    // 表达式配置
    let config = ExpressionConfig::new(None);
    let result = SqlBuilder::new_simplify(pool, feature_name, config, param, id)
        .query_one()
        .await;
    match result {
        Ok(data) => ResResult::with_success(data),
        Err(err) => ResResult::<()>::with_error(&err.to_string()),
    }
}
