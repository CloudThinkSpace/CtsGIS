use axum::response::IntoResponse;

pub async fn search_handler() -> impl IntoResponse {

    "Welcome to CTS GIS Server!".into_response()

}