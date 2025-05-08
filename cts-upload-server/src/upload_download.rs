#![allow(dead_code)] //允许未使用的代码

use cts_common::utils::string::parse_octal_escapes;
use std::collections::HashMap;

use axum::extract::{Multipart, Path};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use cts_common::config::storage::StorageType;
use cts_middleware::extract::config::CtsConfig;
use cts_middleware::extract::db::DbPool;
use cts_sql_expression::expression::save_sql::SaveSqlBuilder;
use cts_upload::OssConfig;
use cts_upload::error::CtsUpLoadError;
use cts_upload::header::{ContentType, create_file_header_map};
use cts_upload::multipart::{write_multipart_file, write_multipart_oss};
use cts_upload::read::CtsReader;
use cts_upload::read::file::CtsFileReader;
use cts_upload::read::oss::CtsOssReader;
use response_utils::res::ResResult;
use serde_json::Value;

pub static NAME: &str = "name";
pub static PATH: &str = "path";
pub static EXT: &str = "ext";

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
        Ok((_, data)) => ResResult::with_success(data),
        Err(err) => ResResult::<()>::with_error(&err.to_string()),
    }
}

/// 上传函数,带表名
pub async fn upload_table(
    DbPool(pool): DbPool,
    CtsConfig(config): CtsConfig,
    Path(table_name): Path<String>,
    multipart: Multipart,
) -> impl IntoResponse {
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
        Ok((data, files)) => {
            // 获取数据库连接池
            let pool = pool.as_ref();
            let schema = config.database.schema.clone();
            let mut new_data = HashMap::new();
            // 遍历数据
            for (key, value) in data {
                new_data.insert(key, Value::String(value));
            }
            let mut file_names = String::from("");
            let mut paths = String::from("");
            let mut exts = String::from("");
            // 遍历文件
            for file in files.iter() {
                file_names.push_str(&format!("{};", parse_octal_escapes(&file.name)));
                paths.push_str(&format!("{};", file.path));
                exts.push_str(&format!("{};", file.ext));
            }

            file_names.pop();
            paths.pop();
            exts.pop();

            new_data.insert(NAME.to_string(), Value::String(file_names));
            new_data.insert(PATH.to_string(), Value::String(paths));
            new_data.insert(EXT.to_string(), Value::String(exts));
            // 配置
            let _ = SaveSqlBuilder::new(
                new_data,
                pool,
                table_name,
                schema.unwrap_or("public".to_string()),
            )
            .execute()
            .await;

            ResResult::with_success(files)
        }
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
