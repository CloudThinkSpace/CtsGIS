use std::sync::Arc;

use crate::route::root_route;
use axum::Extension;
use cts_common::{config::Config, utils::ip::get_local_ip};
use tokio::net::TcpListener;
use tracing::error;

/// @description 服务启动函数
pub async fn start() {
    // 加载配置文件
    let config = match Config::load() {
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
    println!("starting server: http://{}:{}", &server.host, &server.port);
    println!(
        "starting server: http://{}:{}",
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
