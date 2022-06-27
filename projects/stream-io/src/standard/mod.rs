use byteorder::{ReadBytesExt, WriteBytesExt};
use concat_idents::concat_idents;
use std::io::{Read, Result, Write};

use crate::{BigEndian, ByteOrder, LittleEndian};

mod reader;
mod writer;

pub trait StreamReader: Sized {
    /// Reads something from the specified buffer using the specified byte order.
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self>;
}

pub trait StreamWriter: Sized {
    /// Writes something to the specified buffer using the specified byte order.
    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()>;
}
