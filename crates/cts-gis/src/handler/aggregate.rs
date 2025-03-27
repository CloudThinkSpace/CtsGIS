use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use cts_middleware::extract::config::CtsConfig;
use cts_middleware::extract::db::DbPool;
use cts_sql_expression::config::ExpressionConfig;
use cts_sql_expression::expression::sql::SqlBuilder;
use cts_sql_expression::request::CtsParam;
use response_utils::res::ResResult;

pub async fn aggregate_handler(
    DbPool(pool): DbPool,
    CtsConfig(config): CtsConfig,
    Path(table_name): Path<String>,
    Json(param): Json<CtsParam>,
) -> impl IntoResponse {
    // 判断是否有统计参数
    if param.group_by.is_none() {
        return ResResult::<()>::with_error("缺少【统计】参数错误");
    }
    // 获取数据库连接池
    let pool = pool.as_ref();
    let schema = config.database.schema.clone();
    // 配置
    let config = ExpressionConfig::new(schema);
    let result = SqlBuilder::new(pool, table_name, config, param)
        .query()
        .await;
    match result {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(err) => ResResult::<()>::with_error(&err.to_string()),
    }
}
