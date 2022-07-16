mod errors;
mod logger;

pub use errors::{Error, Result};
use logger::IcedLogger;

fn main() {
    IcedLogger::initialize().unwrap();
    log::info!("Hello, world!");
}
