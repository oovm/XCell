use log::{Log, Metadata, Record, SetLoggerError};

/// Backend for the log crate facade
///
/// Formats strings then passes them to a chaenel to be displayed in the gui,
/// this avoids threading issues (logging must be Send+Sync).
pub struct IcedLogger {}

impl IcedLogger {
    pub fn initialize() -> Result<(), SetLoggerError> {
        log::set_boxed_logger(Box::new(IcedLogger {}))
        // log::set_max_level(max_level);
    }
}

impl Log for IcedLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        println!("{:?} {:?}", metadata.level(), metadata.target());
        true
    }

    fn log(&self, record: &Record) {
        println! {"{:#?}", record};
    }

    fn flush(&self) {}
}
