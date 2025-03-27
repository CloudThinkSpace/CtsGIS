use crate::route::root_route;
use cts_common::server::cts_start;

/// @description 服务启动函数
pub async fn start() {
    // 路由
    let app = root_route();
    cts_start(app, "UploadServer", Some("config/upload-config.yml"), false).await;
}
