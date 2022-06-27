pub use byteorder::{BigEndian, LittleEndian};

pub use self::standard::{StreamReader, StreamWriter};

mod standard;

/// `ByteOrder` describes what order to write bytes to the buffer.
#[derive(Copy, Clone)]
pub enum ByteOrder {
    /// Represents big endian byte order (also called network endian).
    /// This is the default order if none is specified.
    BigEndian,
    /// Represents little endian byte order.
    LittleEndian,
}
