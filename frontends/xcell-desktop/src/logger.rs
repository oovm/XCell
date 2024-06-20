//! Tauri 日志模块
//!
//! 提供日志记录功能，适配 Tauri 应用

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use regex::Regex;

/// Tauri 日志记录器
///
/// 格式化日志并输出到控制台
pub struct TauriLogger {
    level: LevelFilter,
    target_filter: Regex,
}

impl Default for TauriLogger {
    fn default() -> Self {
        Self {
            level: LevelFilter::Trace,
            target_filter: Regex::new(r"(wgpu_core|wgpu_hal|naga|iced_wgpu|iced_winit)(::.+)").unwrap(),
        }
    }
}

impl TauriLogger {
    /// 设置日志级别
    pub fn with_target_level(mut self, level: LevelFilter) -> Self {
        self.level = level;
        self
    }

    /// 设置目标过滤器
    pub fn with_target_filter(mut self, target: &str) -> Self {
        if let Ok(o) = Regex::new(target) {
            self.target_filter = o;
        }
        self
    }

    /// 检查是否应该显示该记录
    fn show_record(&self, record: &Record) -> bool {
        if self.target_filter.is_match(record.metadata().target()) {
            return false;
        }
        true
    }

    /// 初始化日志记录器
    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.level);
        log::set_boxed_logger(Box::new(self))?;
        Ok(())
    }
}

impl Log for TauriLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.show_record(record) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
