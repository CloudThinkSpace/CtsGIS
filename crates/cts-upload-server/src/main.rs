mod route;
mod server;
mod upload_download;

use crate::server::start;
use cts_log::init_logger;

#[tokio::main]
async fn main() {
    // 初始化日志
    let _guard = init_logger("debug");
    // 启动服务器
    start().await;
}
