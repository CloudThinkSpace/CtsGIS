pub mod advance;
pub mod pg_bouncer;
pub mod pool;
pub mod ssl;

use std::time::Duration;

use crate::config::database::advance::AdvancedConfig;
use crate::config::database::pg_bouncer::PgBouncerConfig;
use crate::config::database::pool::PoolConfig;
use crate::config::database::ssl::SSLConfig;
use serde::{Deserialize, Serialize};
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub ssl: Option<SSLConfig>,
    pub pg_bouncer: Option<PgBouncerConfig>,
    pub pool: Option<PoolConfig>,
    pub advanced: Option<AdvancedConfig>,
    pub schema: Option<String>,
}

impl DatabaseConfig {
    pub async fn new_pool(&self) -> PgPool {
        // 连接池配置
        let pg_pool: PgPoolOptions = self.into();
        // 数据库配置
        let pg_connect_option: PgConnectOptions = self.into();
        // 连接数据库创建数据库连接池对象
        pg_pool
            .connect_with(pg_connect_option)
            .await
            .expect("连接数据库失败，请检查配置信息")
    }
}

// pg 配置对象转数据库连接池参数对象
impl From<&DatabaseConfig> for PgPoolOptions {
    fn from(value: &DatabaseConfig) -> Self {
        // 创建数据库连接池
        let mut pg_pool = PgPoolOptions::new();
        // 判断是否有poolconfig
        if let Some(pool_config) = &value.pool {
            // 设置 idle
            if let Some(idle) = pool_config.idle_timeout {
                pg_pool = pg_pool.idle_timeout(Duration::from_millis(idle));
            }
            // 设置最大连接
            if let Some(max) = pool_config.maximum_pool_size {
                pg_pool = pg_pool.max_connections(max);
            }
            // 设置最小连接
            if let Some(min) = pool_config.minimum_idle {
                pg_pool = pg_pool.min_connections(min);
            }
            // 设置最大存活时间
            if let Some(max_lifetime) = pool_config.max_lifetime {
                pg_pool = pg_pool.max_lifetime(Duration::from_micros(max_lifetime))
            }
        }

        pg_pool
    }
}

// pg 配置对象转pg连接参数对象
impl From<&DatabaseConfig> for PgConnectOptions {
    fn from(value: &DatabaseConfig) -> Self {
        let mut options = PgConnectOptions::new_without_pgpass();
        // 数据库端口处理
        options = options.port(value.port);
        // 数据库地址
        options = options.host(value.host.as_str());
        // 用户
        options = options.username(value.user.as_str());
        // 密码
        options = options.password(value.password.as_str());
        // 数据库名称
        options = options.database(value.dbname.as_str());
        // 设置应用名称
        options = options.application_name("CTS GIS Server");
        options
    }
}
