use axum::extract::Path;
use axum::response::IntoResponse;

pub async fn feature_handler(
    Path(feature_name): Path<String>,
) -> impl IntoResponse {

    println!("{:#?}", feature_name);
    "Welcome to CTS GIS Server!".into_response()

}

pub async fn feature_id_handler(
    Path(feature_name): Path<String>,
    Path(id): Path<String>,
) -> impl IntoResponse {

    println!("{:#?},{}", feature_name, id);
    "Welcome to CTS GIS Server!".into_response()

}