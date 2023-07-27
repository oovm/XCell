

use std::process::Command;




use tracing::level_filters::LevelFilter;
use tracing_subscriber::field::MakeExt;
use tracing_subscriber::fmt::time::LocalTime;


pub fn pause() {
    if cfg!(debug_assertions) {
        return;
    }
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    } else {
        let _ = Command::new("pause").status();
    }
}

pub fn logger() {
    if cfg!(debug_assertions) {
        let _ = tracing_subscriber::fmt().with_max_level(LevelFilter::TRACE).try_init();
    } else {
        let _ = tracing_subscriber::fmt()
            .with_max_level(LevelFilter::TRACE)
            .with_timer(
                LocalTime::rfc_3339()
            )
            .fmt_fields(tracing_subscriber::fmt::format::debug_fn(|writer, field, value| write!(writer, "{}: {:?}", field, value))
                .delimited(", "))
            .try_init();
    }
}

// pub fn log_writer(w: &mut Formatter, record: &Record) -> std::io::Result<()> {
//     let header = match record.level() {
//         Level::Error => "Error".bright_red(),
//         Level::Warn => "Warn ".bright_yellow(),
//         Level::Info => "Info ".bright_green(),
//         Level::Debug => "Debug".bright_purple(),
//         Level::Trace => "Trace".bright_magenta(),
//     };
//     let logs = format!("[{header} {}] {}", Local::now().format("%Y-%d-%m %H:%M:%S"), record.args());
//     for (i, line) in logs.lines().enumerate() {
//         if i != 0 {
//             w.write(b"\n")?;
//             w.write(b"    ")?;
//         }
//         w.write(line.as_bytes())?;
//     }
//     w.write(b"\n")?;
//     Ok(())
// }
