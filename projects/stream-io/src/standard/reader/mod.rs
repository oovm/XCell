
use super::*;

#[allow(unused_variables)]
impl StreamReader for u8 {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        buffer.read_u8()
    }
}


impl StreamReader for u16 {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        match order {
            ByteOrder::BigEndian => buffer.read_u16::<BigEndian>(),
            ByteOrder::LittleEndian => buffer.read_u16::<LittleEndian>(),
        }
    }
}