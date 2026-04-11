use std::{
    fmt::{Debug, Write},
    process::Command,
};
use tracing::{Event, field::Field};
use tracing_subscriber::{
    EnvFilter, Registry,
    field::Visit,
    fmt::{FmtContext, FormatEvent, FormatFields, format::Writer},
};

/// 在发布模式下暂停程序等待用户按键
pub fn pause() {
    if cfg!(debug_assertions) {
        return;
    }
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    }
    else {
        let _ = Command::new("pause").status();
    }
}

/// 初始化日志系统，根据参数设置日志级别和输出文件
pub fn logger(verbose: bool, quiet: bool, log_file: &str) {
    let level = if verbose {
        "trace"
    } else if quiet {
        "error"
    } else {
        "info"
    };
    let filter = EnvFilter::try_new(level).unwrap_or_else(|_| EnvFilter::from_default_env());
    
    if !log_file.is_empty() {
        // 输出到文件
        let file_appender = tracing_appender::rolling::never(".", log_file);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        let _ = tracing_subscriber::fmt()
            .event_format(XCellFormat {})
            .with_env_filter(filter)
            .with_writer(non_blocking)
            .try_init();
    } else {
        // 输出到控制台
        let _ = tracing_subscriber::fmt()
            .event_format(XCellFormat {})
            .with_env_filter(filter)
            .try_init();
    }
}

struct XCellFormat {}

struct FieldsVisitor<'a> {
    writer: Writer<'a>,
}

impl<'a> Write for FieldsVisitor<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.writer.write_str(s)
    }
}

impl<N> FormatEvent<Registry, N> for XCellFormat
where
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(&self, _: &FmtContext<'_, Registry, N>, writer: Writer<'_>, event: &Event<'_>) -> std::fmt::Result {
        let meta = event.metadata();
        let mut f = FieldsVisitor { writer };
        for _ in event.fields() {
            f.write_str("\x1b[37m[")?;
            match meta.level().as_str() {
                "TRACE" => f.write_str("\x1b[34mTrace")?,
                "DEBUG" => f.write_str("\x1b[36mDebug")?,
                "INFO" => f.write_str("\x1b[32mDebug")?,
                "WARN" => f.write_str("\x1b[33mAlert")?,
                _ => f.write_str("\x1b[31mError")?,
            }
            f.write_str(" \x1b[37m")?;
            let now = chrono::Local::now();
            write!(f, "{}", now.format("%m-%d %H:%M:%S"))?;
            f.write_str("\x1b[37m]\x1b[0m ")?;
            #[cfg(debug_assertions)]
            match (meta.module_path(), meta.line()) {
                (Some(path), Some(line)) => {
                    write!(f, "{}:{} ", path, line)?;
                }
                _ => {}
            }
            event.record(&mut f);
        }
        f.write_char('\n')
    }
}

impl<'w> Visit for FieldsVisitor<'w> {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        match field.name() {
            "message" => {
                let _ = write!(self.writer, "{:?} ", value);
            }
            _ => {
                if cfg!(debug_assertions) {
                    let _ = write!(self.writer, "\n{}={:?} ", field.name(), value);
                }
            }
        }
    }
}
