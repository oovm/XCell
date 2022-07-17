use std::borrow::Borrow;

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use regex::Regex;

/// Backend for the log crate facade
///
/// Formats strings then passes them to a chaenel to be displayed in the gui,
/// this avoids threading issues (logging must be Send+Sync).
pub struct IcedLogger {
    level: LevelFilter,
    target_filter: Regex,
}

impl Default for IcedLogger {
    fn default() -> Self {
        Self {
            level: LevelFilter::Trace,
            target_filter: Regex::new(r"(wgpu_core|wgpu_hal|naga|iced_wgpu|iced_winit)(::.+)").unwrap(),
        }
    }
}
impl IcedLogger {
    fn show_record(&self, record: &Record) -> bool {
        if self.target_filter.is_match(record.metadata().target()) {
            return false;
        }
        true
    }
}

impl IcedLogger {
    pub fn activate(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.level);
        log::set_boxed_logger(Box::new(self))?;
        Ok(())
    }
}

impl Log for IcedLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        //         metadata.level() <= Level::Info
        println!("{:?} {:?}", metadata.level(), metadata.target());
        true
    }
    fn log(&self, record: &Record) {
        if self.show_record(record) {
            println! {"{:#?}", record};
        }
    }
    fn flush(&self) {}
}
