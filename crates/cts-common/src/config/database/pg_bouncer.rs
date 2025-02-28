use serde::{Deserialize, Serialize};

/// # pgBouncer 专用属性
/// - `pool_mode`: 池模式：session（会话级）、transaction（事务级）、statement（语句级）,默认值transaction。
/// - `max_client_conn`: 允许的最大客户端连接数，默认值：100。
/// - `default_pool_size`: 每个数据库-用户组合的默认连接数， 默认值：20。
/// - `reserve_pool_size`: 保留的连接数（用于突发请求），默认值：5。
/// - `ignore_startup_parameters`: 忽略客户端指定的某些参数（优化兼容性），默认值：extra_float_digits。
#[derive(Serialize, Deserialize, Debug)]
pub struct PgBouncerConfig {
    pub pool_mode: Option<bool>,
    pub max_client_conn: Option<u32>,
    pub default_pool_size: Option<u32>,
    pub reserve_pool_size: Option<u32>,
    pub ignore_startup_parameters: Option<String>,
}
