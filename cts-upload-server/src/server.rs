use crate::route::root_route;
use axum::Extension;
use cts_common::{config::Config, utils::ip::get_local_ip};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::error;
use cts_common::license::license;

/// @description 服务启动函数
pub async fn start() {
    // 许可检查
    license("UploadServer");
    // 加载配置文件
    let config = match Config::load_with_config("config/upload-config.yml") {
        Ok(data) => data,
        Err(err) => {
            error!("服务配置错误：{}", err);
            return;
        }
    };
    // 服务器配置
    let server = &config.server;
    // 打印服务器信息
    println!("starting server: http://{}:{}", &server.host, &server.port);
    println!(
        "starting server: http://{}:{}",
        get_local_ip().unwrap(),
        &server.port
    );
    // 路由
    let app = root_route()
        .layer(Extension(Arc::new(config.clone())));
    // 监听服务
    let listener = TcpListener::bind(format!("{}:{}", server.host, server.port))
        .await
        .unwrap();
    // 启动服务
    axum::serve(listener, app).await.unwrap();
}
