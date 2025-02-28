use axum::extract::Path;
use axum::response::IntoResponse;

pub async fn search_handler(Path(table_name): Path<String>) -> impl IntoResponse {
    println!("{:#?}", table_name);
    "Welcome to CTS GIS Server!".into_response()
}

pub async fn query_handler(
    Path(table_name): Path<String>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    println!("{:#?}, {}", table_name, id);
    "Welcome to CTS GIS Server!".into_response()
}
