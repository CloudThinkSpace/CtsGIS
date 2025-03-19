#![allow(dead_code)] //允许未使用的代码

use axum::extract::{Multipart, Path};
use axum::http::{HeaderMap, HeaderName, HeaderValue};
use axum::response::IntoResponse;
use cts_upload::multipart::write_multipart_file;
use cts_upload::read::file::CtsFileReader;
use cts_upload::read::CtsReader;
use response_utils::res::ResResult;

/// 上传函数
pub async fn upload(multipart: Multipart) -> impl IntoResponse {
    let result = write_multipart_file(multipart, "upload").await;

    match result {
        Ok(data) => ResResult::with_success(data),
        Err(_err) => ResResult::<()>::with_error(&_err.to_string()),
    }
}

/// 浏览文件函数
pub async fn image(Path(path): Path<String>) -> (HeaderMap, Vec<u8>) {
    let reader = CtsFileReader(path.clone());
    let result = reader.read().await;
    let headers = parse_type(path, FileType::Image);
    match result {
        Ok(data) => (headers, data),
        Err(err) => (headers, err.to_string().into_bytes()),
    }
}

/// 下载文件
pub async fn download(Path(path): Path<String>) -> (HeaderMap, Vec<u8>) {
    let reader = CtsFileReader(path.clone());
    let result = reader.read().await;
    let headers = parse_type(path, FileType::Other);
    match result {
        Ok(data) => (headers, data),
        Err(err) => (headers, err.to_string().into_bytes()),
    }
}

fn parse_type(path: String, file_type: FileType) -> HeaderMap {
    // 查找是否有点符号
    let index = path.find('.').unwrap_or(usize::MAX);
    //文件扩展名
    let mut ext_name = None;
    if index != usize::MAX {
        ext_name = Some(&path[index + 1..]);
    }
    let content_type = match file_type {
        FileType::Image => {
            format!("image/{}", ext_name.unwrap())
        }
        FileType::Video => "video/*".to_string(),
        FileType::Other => "application/octet-stream".to_string(),
    };
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_str(&content_type).unwrap(),
    );
    headers
}

pub enum FileType {
    Image,
    Video,
    Other,
}
