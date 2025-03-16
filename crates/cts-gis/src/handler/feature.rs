use axum::extract::Path;
use axum::response::IntoResponse;
use cts_middleware::extract::db::DbPool;

pub async fn feature_handler(
    DbPool(pool): DbPool,
    Path(feature_name): Path<String>,
) -> impl IntoResponse {
    let aa = sqlx::query("select * from pct")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();
    println!("{:?}", aa);
    "Welcome to CTS GIS Server!".into_response()
}

pub async fn feature_id_handler(
    Path(feature_name): Path<String>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    println!("{:#?},{}", feature_name, id);
    "Welcome to CTS GIS Server!".into_response()
}
