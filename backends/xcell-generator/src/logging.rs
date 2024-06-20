use tracing::{debug, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// 日志配置
pub struct LogConfig {
    /// 日志级别
    pub level: LevelFilter,
    /// 是否启用彩色输出
    pub colored: bool,
    /// 是否显示时间戳
    pub timestamp: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self { level: LevelFilter::INFO, colored: true, timestamp: true }
    }
}

/// 初始化日志系统
pub fn init_logging(config: LogConfig) {
    let env_filter = EnvFilter::builder().with_default_directive(config.level.into()).from_env_lossy();

    let fmt_layer =
        fmt::layer().with_target(true).with_level(true).with_timer(fmt::time::LocalTime::rfc_3339()).with_ansi(config.colored);

    let subscriber = tracing_subscriber::registry().with(env_filter).with(fmt_layer);

    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        // 全局 subscriber 已经设置，忽略错误
        debug!("Global tracing subscriber already set: {:?}", e);
    }
}

/// 日志级别
pub enum LogLevel {
    /// 错误
    Error,
    /// 警告
    Warn,
    /// 信息
    Info,
    /// 调试
    Debug,
    /// 跟踪
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => LevelFilter::ERROR,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Trace => LevelFilter::TRACE,
        }
    }
}
