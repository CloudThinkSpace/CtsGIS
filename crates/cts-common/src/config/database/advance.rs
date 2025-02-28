use serde::{Deserialize, Serialize};

/// # 高级配置与优化
/// - `pool_name`: 连接池名称（用于监控和日志）。
/// - `auto_commit`: 是否自动提交事务（建议关闭，由应用显式控制事务）。
/// - `read_only`: 是否将连接设为只读模式（适用于只读查询场景）。
/// - `test_on_borrow`: 从池中借用连接时是否验证其有效性（可能影响性能，建议异步验证）。
/// - `test_while_idle`: 空闲时是否定期验证连接有效性（推荐开启）。
#[derive(Serialize, Deserialize, Debug)]
pub struct AdvancedConfig {
    pub pool_name: Option<String>,
    pub auto_commit: Option<bool>,
    pub read_only: Option<bool>,
    pub test_on_borrow: Option<bool>,
    pub test_while_idle: Option<bool>,
}
