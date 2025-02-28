use serde::{Deserialize, Serialize};

/// SSL 加密相关
/// - `ssl_mode`:  控制 SSL 安全模式，可选值：disable/allow/prefer/require/verify-ca/verify-full。
/// - `ssl_root_cert`: SSL CA 证书路径（用于验证服务器证书）。
/// - `ssl_cert`:  客户端证书路径（双向认证）。
/// - `ssl_key`:   客户端私钥路径（双向认证）。
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SSLConfig {
    pub ssl_mode: Option<String>,
    pub ssl_root_cert: Option<String>,
    pub ssl_cert: Option<String>,
    pub ssl_key: Option<String>,
}
