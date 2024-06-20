use concat_idents::concat_idents;

use super::*;

impl StreamReader for () {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        Ok(())
    }
}

impl StreamReader for bool {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        match buffer.read_u8()? {
            0 => Ok(true),
            _ => Ok(false),
        }
    }
}

impl StreamReader for u8 {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        buffer.read_u8()
    }
}

impl StreamReader for i8 {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        buffer.read_i8()
    }
}

macro_rules! r_number {
    ($($t:ty),*) => {
        $(
            impl StreamReader for $t {
                fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
                    concat_idents!(read_fn = read_, $t {
                        match order {
                            ByteOrder::BigEndian => buffer.read_fn::<BigEndian>(),
                            ByteOrder::LittleEndian => buffer.read_fn::<LittleEndian>(),
                        }
                    })
                }
            }
        )*
    };
}

r_number![u16, u32, u64, u128];
r_number![i16, i32, i64, i128];
r_number![f32, f64];

impl<T> StreamReader for Vec<T>
where
    T: StreamReader,
{
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        let mut out = <Vec<T>>::with_capacity(u32::read_from(buffer, order)? as usize);
        for _ in 0..out.capacity() {
            out.push(T::read_from(buffer, order)?);
        }
        Ok(out)
    }
}

impl<T> StreamReader for VecDeque<T>
where
    T: StreamReader,
{
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        let mut out = <VecDeque<T>>::with_capacity(u32::read_from(buffer, order)? as usize);
        for _ in 0..out.capacity() {
            out.push_back(T::read_from(buffer, order)?);
        }
        Ok(out)
    }
}
