use serde::{Deserialize, Serialize};

// JWT 密钥
pub const SECRET: &str = "CLOUDTHINKSPACE";

// 声明 Token 结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // 用户ID
    pub exp: usize,   // 过期时间戳
    pub role: String, // 用户角色
}

// 错误响应
#[derive(Serialize)]
pub struct AuthError {
    pub message: String,
}
