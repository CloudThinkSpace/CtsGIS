#![allow(dead_code)] //允许未使用的代码

use axum::extract::{Multipart, Path};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use cts_common::config::storage::StorageType;
use cts_middleware::extract::config::CtsConfig;
use cts_upload::error::CtsUpLoadError;
use cts_upload::header::{create_file_header_map, ContentType};
use cts_upload::multipart::{write_multipart_file, write_multipart_oss};
use cts_upload::read::file::CtsFileReader;
use cts_upload::read::oss::CtsOssReader;
use cts_upload::read::CtsReader;
use cts_upload::OssConfig;
use response_utils::res::ResResult;

/// 上传函数
pub async fn upload(CtsConfig(config): CtsConfig, multipart: Multipart) -> impl IntoResponse {
    let result = match &config.storage {
        None => write_multipart_file(multipart, "upload").await,
        Some(storage_config) => {
            match storage_config.r#type {
                StorageType::OSS => {
                    // 解析配置信息
                    let oss_config = storage_config.parse_config::<OssConfig>();
                    match oss_config {
                        None => Err(CtsUpLoadError::WriteError(
                            "oss对象存储缺少关键配置【key_id，key_secret，endpoint，bucket】"
                                .to_string(),
                        )),
                        Some(data) => {
                            write_multipart_oss(multipart, &storage_config.path, data).await
                        }
                    }
                }
                StorageType::FILE => write_multipart_file(multipart, &storage_config.path).await,
                _ => write_multipart_file(multipart, "upload").await,
            }
        }
    };

    match result {
        Ok(data) => ResResult::with_success(data),
        Err(err) => ResResult::<()>::with_error(&err.to_string()),
    }
}

/// 浏览文件函数
pub async fn image(CtsConfig(config): CtsConfig, Path(path): Path<String>) -> (HeaderMap, Vec<u8>) {
    let headers = create_file_header_map(path.to_string(), ContentType::Image);
    let result = match &config.storage {
        None => {
            let reader = CtsFileReader(path.to_string());
            reader.read().await
        }
        Some(storage_config) => {
            match storage_config.r#type {
                StorageType::OSS => {
                    // 解析配置信息
                    let oss_config = storage_config.parse_config::<OssConfig>();
                    match oss_config {
                        None => Err(CtsUpLoadError::ReadError(
                            "oss对象存储缺少关键配置【key_id，key_secret，endpoint，bucket】"
                                .to_string(),
                        )),
                        Some(data) => {
                            let reader = CtsOssReader::new(path.clone(), data);
                            reader.read().await
                        }
                    }
                }
                StorageType::FILE => {
                    let reader = CtsFileReader(path.clone());
                    reader.read().await
                }
                _ => {
                    let reader = CtsFileReader("upload".to_string());
                    reader.read().await
                }
            }
        }
    };

    match result {
        Ok(data) => (headers, data),
        Err(err) => (headers, err.to_string().into_bytes()),
    }
}

/// 下载文件
pub async fn download(
    CtsConfig(_config): CtsConfig,
    Path(path): Path<String>,
) -> (HeaderMap, Vec<u8>) {
    let reader = CtsFileReader(path.clone());
    let result = reader.read().await;
    let headers = create_file_header_map(path, ContentType::Other);
    match result {
        Ok(data) => (headers, data),
        Err(err) => (headers, err.to_string().into_bytes()),
    }
}
