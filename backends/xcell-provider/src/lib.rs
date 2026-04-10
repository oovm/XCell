#![warn(missing_docs)]
pub use byteorder::{BigEndian, LittleEndian};
pub use xcell_core::{XError, XErrorKind};
pub type XResult<T> = Result<T, XError>;

pub use self::{
    standard::{StreamReader, StreamWriter},
    table::{
        CsvRows, CsvTable, ExcelTable, FileFormat, FileFormatDetector, TableReader, TsvRows, TsvTable, XCellAccess,
        XCellHeader, XDocument, load_table,
    },
};

mod standard;
pub mod table;

/// `ByteOrder` describes what order to write bytes to the buffer.
#[derive(Copy, Clone)]
pub enum ByteOrder {
    /// Represents big endian byte order (also called network endian).
    /// This is the default order if none is specified.
    BigEndian,
    /// Represents little endian byte order.
    LittleEndian,
}
