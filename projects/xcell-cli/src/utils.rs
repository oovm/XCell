use std::io::Write;

use env_logger::{builder, fmt::Formatter};
use log::{Level, LevelFilter, Record};

use chrono::Local;
use colored::Colorize;

use super::*;

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

pub fn logger() {
    if cfg!(debug_assertions) {
        let _ = builder().filter(Some("globset"), LevelFilter::Off).filter_level(LevelFilter::Trace).try_init();
    }
    else {
        let _ = builder()
            .format(log_writter)
            .filter(Some("globset"), LevelFilter::Off)
            .filter_level(LevelFilter::Trace)
            // .is_test(cfg!(debug_assertions))
            .try_init();
    }
}

pub fn log_writter(w: &mut Formatter, record: &Record) -> std::io::Result<()> {
    let header = match record.level() {
        Level::Error => "Error".bright_red(),
        Level::Warn => "Warn ".bright_yellow(),
        Level::Info => "Print".bright_green(),
        Level::Debug => "Debug".bright_purple(),
        Level::Trace => "Trace".bright_magenta(),
    };
    let logs = format!("[{header} {}] {}", Local::now().format("%Y-%d-%m %H:%M:%S"), record.args());
    for (i, line) in logs.lines().enumerate() {
        if i != 0 {
            w.write(b"\n")?;
            w.write(b"    ")?;
        }
        w.write(line.as_bytes())?;
    }
    w.write(b"\n")?;
    Ok(())
}
