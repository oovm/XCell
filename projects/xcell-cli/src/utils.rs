use std::fmt::Debug;
use std::process::Command;
use tracing::field::Field;


use tracing::level_filters::LevelFilter;
use tracing::span::Record;
use tracing_subscriber::field::{MakeExt, RecordFields, Visit, VisitOutput};
use tracing_subscriber::fmt::format::{PrettyVisitor, Writer};
use tracing_subscriber::fmt::{FormatFields, FormattedFields};
use tracing_subscriber::fmt::time::{FormatTime, LocalTime};


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
    let _ = tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .with_timer(XCellTimer {})
        .event_format(XCellFields {})
        .try_init();
}

struct XCellTimer {}

struct XCellFields {}

struct XCellFieldVisitor<'writer> {
    writer: Writer<'writer>,
}

impl FormatTime for XCellTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = chrono::Local::now();
        write!(w, "{}", now.format("%m-%d %H:%M:%S"))
    }
}

impl<'writer> FormatFields<'writer> for XCellFields {
    fn format_fields<R: RecordFields>(&self, writer: Writer<'writer>, fields: R) -> std::fmt::Result {
        let mut v = XCellFieldVisitor { writer };
        fields.record(&mut v);
        Ok(())
    }

    fn add_fields(&self, current: &'writer mut FormattedFields<Self>, fields: &Record<'_>) -> std::fmt::Result {
        let empty = current.is_empty();
        let writer = current.as_writer();
        let mut v = XCellFieldVisitor { writer };
        fields.record(&mut v);
        Ok(())
    }
}

impl<'writer> Visit for XCellFieldVisitor<'writer> {

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        match field.name() {
            "message" => {
                let _ = write!(self.writer, "\n{:?}\n", value);
            }
            _ => {
                // let _ = write!(self.writer, "{}={:?} ", field.name(), value);
            }
        }
    }
}