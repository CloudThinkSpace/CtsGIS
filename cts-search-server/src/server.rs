use std::sync::Arc;

use axum::Extension;
use cts_common::{config::Config, utils::ip::get_local_ip};
use tokio::net::TcpListener;
use tracing::error;
use cts_common::license::license;
use crate::route::root_route;

/// @description 服务启动函数
pub async fn start() {
    // 许可检查
    license("SearchServer");
    // 加载配置文件
    let config = match Config::load_with_config("config/search-config.yml")  {
        Ok(data) => data,
        Err(err) => {
            error!("{}", err);
            return;
        }
    };
    // 服务器配置
    let server = &config.server;
    let database = &config.database;
    // 数据库连接池
    let pg_pool = database.new_pool().await;
    // 打印服务器信息
    println!("Starting Search Server: http://{}:{}", &server.host, &server.port);
    println!(
        "Starting Search Server: http://{}:{}",
        get_local_ip().unwrap(),
        &server.port
    );
    // 路由
    let app = root_route()
        .layer(Extension(Arc::new(pg_pool)))
        .layer(Extension(Arc::new(config.clone())));
    // 监听服务
    let listener = TcpListener::bind(format!("{}:{}", server.host, server.port))
        .await
        .unwrap();
    // 启动服务
    axum::serve(listener, app).await.unwrap();
}
