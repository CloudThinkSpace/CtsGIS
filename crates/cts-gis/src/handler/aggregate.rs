use axum::response::IntoResponse;

pub async fn aggregate_handler() -> impl IntoResponse {

    "Welcome to CTS GIS Server!".into_response()

}