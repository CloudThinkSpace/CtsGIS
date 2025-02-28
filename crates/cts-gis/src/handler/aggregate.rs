use axum::extract::Path;
use axum::response::IntoResponse;

pub async fn aggregate_handler(Path(table_name): Path<String>) -> impl IntoResponse {
    println!("{}", table_name);
    "Welcome to CTS GIS Server!".into_response()
}
