use axum::extract::Path;
use axum::response::IntoResponse;

pub async fn feature_handler(
    Path(feature_id): Path<String>,
) -> impl IntoResponse {

    println!("{:#?}", feature_id);
    "Welcome to CTS GIS Server!".into_response()

}