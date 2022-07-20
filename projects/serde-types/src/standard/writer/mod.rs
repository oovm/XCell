use super::*;
use std::collections::VecDeque;

use concat_idents::concat_idents;

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
                    concat_idents!(write_fn = write_, $t {
                        match order {
                            ByteOrder::BigEndian => buffer.write_fn::<BigEndian>(*self),
                            ByteOrder::LittleEndian => buffer.write_fn::<LittleEndian>(*self),
                        }
                    })
                }
            }
        )*
    };
}

w_number![u16, u32, u64, u128];
w_number![i16, i32, i64, i128];
w_number![f32, f64];

macro_rules! w_vector {
    ($($t:ty),*) => {
        $(
            impl<T> StreamWriter for $t
            where
                T: StreamWriter
            {
                fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
                    (self.len() as u32).write_to(buffer, order)?;
                    for item in self {
                        item.write_to(buffer, order)?;
                    }
                    Ok(())
                }
            }
        )*
    };
}

w_vector![Vec<T>, VecDeque<T>];
