use serde::{Deserialize, Serialize};

/// # 通用连接池属性
/// - `minimum_idle`: 连接池中保持的最小空闲连接数（避免频繁创建新连接）,默认值：5。
/// - `maximum_pool_size`: 连接池允许的最大连接数（根据数据库负载和应用需求调整），默认值：20
/// - `connection_timeout`: 获取连接的超时时间（单位：毫秒，超时抛出异常）,默认值：30000。
/// - `idle_timeout`: 空闲连接被回收的时间（单位：毫秒，超时关闭连接）, 默认值：600000
/// - `max_lifetime`: 连接的最大生存时间（单位：毫秒，避免长时间占用连接导致资源泄漏），默认值：1800000。
/// - `validation_query`: 验证连接是否有效的 SQL 语句（如简单查询 SELECT 1）。
/// - `leak_detection_threshold`: 连接泄漏检测阈值（单位：毫秒，超时未归还连接则报警），默认值：5000。
/// - `keepalives`: 是否启用 TCP 保活机制（1 启用，0 禁用）,默认值：1。
/// - `keepalives_idle`: 保活探测间隔（单位：秒），默认值：60。
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoolConfig {
    pub minimum_idle: Option<u32>,
    pub maximum_pool_size: Option<u32>,
    pub connection_timeout: Option<u32>,
    pub idle_timeout: Option<u32>,
    pub max_lifetime: Option<u32>,
    pub validation_query: Option<String>,
    pub leak_detection_threshold: Option<u32>,
    pub keepalives: Option<bool>,
    pub keepalives_idle: Option<u32>,
}
