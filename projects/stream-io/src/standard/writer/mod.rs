use super::*;

#[allow(unused_variables)]
impl StreamWriter for u8 {
    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        buffer.write_u8(*self)
    }
}
#[allow(unused_variables)]
impl StreamWriter for i8 {
    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        buffer.write_i8(*self)
    }
}

macro_rules! w_number {
    ($($t:ty),*) => {
        $(
            impl StreamWriter for $t {
                fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
                    match order {
                        ByteOrder::BigEndian => buffer.concat_idents!(write_, $t)::<BigEndian>(*self),
                        ByteOrder::LittleEndian => buffer.concat_idents!(write_, $t)::<LittleEndian>(*self),
                    }
                }
            }
        )*
    };
}

w_number![u16, u32, u64, u128];
