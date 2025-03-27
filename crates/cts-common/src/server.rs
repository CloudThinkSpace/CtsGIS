use crate::config::Config;
use crate::license::license;
use crate::utils::ip::get_local_ip;
use axum::{Extension, Router};
use log::error;
use std::sync::Arc;
use tokio::net::TcpListener;

/// ## 启动服务
/// @param `app` 自定义router
/// @param `server_name` 服务名称
/// @param `config_option` 是否有配置路径
/// @param `load_pool` 是否加载连接池
pub async fn cts_start(app: Router, server_name: &str, config_option: Option<&str>, load_pool: bool) {
    // 许可检查
    license(server_name);
    // 加载配置文件
    let config = match config_option {
        None => match Config::load() {
            Ok(data) => data,
            Err(err) => {
                error!("{}", err);
                return;
            }
        },
        Some(config_path) => match Config::load_with_config(config_path) {
            Ok(data) => data,
            Err(err) => {
                error!("{}", err);
                return;
            }
        },
    };

    // 服务器配置
    let server = &config.server;
    let database = &config.database;

    // 打印服务器信息
    println!(
        "Starting {}: http://{}:{}",
        server_name, &server.host, &server.port
    );
    println!(
        "Starting {}: http://{}:{}",
        server_name,
        get_local_ip().unwrap(),
        &server.port
    );
    // 路由
    let app = match load_pool {
        true => {
            // 数据库连接池
            let pg_pool = database.new_pool().await;
            app.layer(Extension(Arc::new(pg_pool)))
                .layer(Extension(Arc::new(config.clone())))
        }
        false => app.layer(Extension(Arc::new(config.clone()))),
    };

    // 监听服务
    let listener = TcpListener::bind(format!("{}:{}", server.host, server.port))
        .await
        .unwrap();
    // 启动服务
    axum::serve(listener, app).await.unwrap();
}
