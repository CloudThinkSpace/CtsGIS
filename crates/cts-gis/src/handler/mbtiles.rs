use axum::extract::Path;
use axum::response::IntoResponse;

pub async fn mbtiles_handler(
    Path(tiles): Path<String>,
) -> impl IntoResponse {

    println!("{:#?}", tiles);
    "Welcome to CTS GIS Server!".into_response()

}