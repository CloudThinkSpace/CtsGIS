use std::fs::read;
use axum::extract::Path;
use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};

pub async fn static_data(
    Path(file_path): Path<String>,
) -> Response {
    let bytes = read(format!("./data/{file_path}"));
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_str("application/octet-stream").unwrap(),
    );
    match bytes {
        Ok(data) => (
            [
                ("Access-Control-Allow-Origin", "*"),
                ("content-type", "application/octet-stream"),
            ]
            , data
        ).into_response(),
        Err(_err) => (StatusCode::SERVICE_UNAVAILABLE,"无法访问服务器资源").into_response(),
    }
}