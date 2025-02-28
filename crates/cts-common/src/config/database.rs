pub mod advance;
pub mod pg_bouncer;
pub mod pool;
pub mod ssl;

use crate::config::database::advance::AdvancedConfig;
use crate::config::database::pg_bouncer::PgBouncerConfig;
use crate::config::database::pool::PoolConfig;
use crate::config::database::ssl::SSLConfig;
use serde::{Deserialize, Serialize};

///
/// - `host`: 数据库服务器地址（IP 或域名）。
/// - `port`: 数据库服务端口（默认 5432）。
/// - `user`: 连接数据库的用户名。
/// - `password`: 用户的密码（建议通过安全方式传递，避免明文存储）。
/// - `dbname`: 要连接的数据库名称。
/// - `ssl`: SSL 加密相关配置
/// - `pg_bouncer`: pgBouncer 专用属性配置
/// - `pool`: 数据库连接池配置
/// - `advanced`: 高级配置
#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub ssl: Option<SSLConfig>,
    pub pg_bouncer: Option<PgBouncerConfig>,
    pub pool: Option<PoolConfig>,
    pub advanced: Option<AdvancedConfig>,
}
