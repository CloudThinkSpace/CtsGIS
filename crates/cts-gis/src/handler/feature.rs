use axum::extract::Path;
use axum::response::IntoResponse;
use cts_middleware::extract::db::DbPool;
use cts_pgrow::SerMapPgRow;
use response_utils::res::ResResult;
use serde_json::Value;
use sqlx::QueryBuilder;

pub async fn feature_handler(
    DbPool(pool): DbPool,
    Path(feature_name): Path<String>,
) -> impl IntoResponse {
    let mut builder = QueryBuilder::new("select * from ");
    let query = builder.push(feature_name).build();
    let rows = query.fetch_all(pool.as_ref()).await;
    match rows {
        Ok(data) => {
            let mut result = Vec::new();
            for row in data.into_iter() {
                let map = SerMapPgRow::from(row);
                let data: Value = map.into();
                result.push(data);
            }
            ResResult::with_success(result)
        }
        Err(err) => ResResult::<()>::with_error(&err.to_string()),
    }
}

pub async fn feature_id_handler(
    DbPool(pool): DbPool,
    Path((feature_name, id)): Path<(String, String)>,
) -> impl IntoResponse {
    let mut builder = QueryBuilder::new("select * from ");
    builder.push(feature_name);
    builder.push(" where id = ");
    builder.push_bind(id);
    let query = builder.build();
    let row = query.fetch_one(pool.as_ref()).await;
    match row {
        Ok(data) => {
            let map = SerMapPgRow::from(data);
            let data: Value = map.into();
            ResResult::with_success(data)
        }
        Err(err) => ResResult::<()>::with_error(&err.to_string()),
    }
}
