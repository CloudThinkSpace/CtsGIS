use axum::response::IntoResponse;

pub async fn add_user() -> impl IntoResponse {
    "aaaa".into_response()
}
