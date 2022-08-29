use env_logger::builder;
use log::LevelFilter;

use super::*;

pub fn logger() {
    if cfg!(debug_assertions) {
        let _ = builder().filter_level(LevelFilter::Trace).try_init();
    }
    else {
        let _ = builder().filter_level(LevelFilter::Info).try_init();
    }
}

pub fn pause() -> XResult {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    }
    else {
        let _ = Command::new("pause").status();
    }
    Ok(())
}
