use axum::response::IntoResponse;

pub async fn mbtiles_handler() -> impl IntoResponse {

    "Welcome to CTS GIS Server!".into_response()

}