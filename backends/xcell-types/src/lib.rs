#![warn(missing_docs)]
#![allow(clippy::get_first)]

/// 字节序枚举，描述写入缓冲区的字节顺序。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ByteOrder {
    /// 表示大端字节序（也称为网络字节序）。
    /// 如果未指定，这是默认顺序。
    BigEndian,
    /// 表示小端字节序。
    LittleEndian,
}

/// StreamReader 特性，用于使用指定的字节序从缓冲区读取数据。
pub trait StreamReader: Sized {
    /// 使用指定的字节序从指定的缓冲区读取数据。
    ///
    /// # 参数
    /// * `buffer` - 要读取的缓冲区
    /// * `order` - 使用的字节序
    ///
    /// # 返回值
    /// 返回读取的结果，成功时返回 Self，失败时返回 std::io::Error。
    fn read_from<R: std::io::Read>(buffer: &mut R, order: ByteOrder) -> std::io::Result<Self>;
}

/// StreamWriter 特性，用于使用指定的字节序向缓冲区写入数据。
pub trait StreamWriter: Sized {
    /// 使用指定的字节序向指定的缓冲区写入数据。
    ///
    /// # 参数
    /// * `buffer` - 要写入的缓冲区
    /// * `order` - 使用的字节序
    ///
    /// # 返回值
    /// 返回写入的结果，成功时返回 Ok(())，失败时返回 std::io::Error。
    fn write_to<W: std::io::Write>(&self, buffer: &mut W, order: ByteOrder) -> std::io::Result<()>;
}

impl StreamWriter for u8 {
    fn write_to<W: std::io::Write>(&self, buffer: &mut W, _order: ByteOrder) -> std::io::Result<()> {
        buffer.write_all(&[*self])
    }
}

impl StreamWriter for i8 {
    fn write_to<W: std::io::Write>(&self, buffer: &mut W, _order: ByteOrder) -> std::io::Result<()> {
        buffer.write_all(&[*self as u8])
    }
}

macro_rules! w_number {
    ($($t:ty),*) => {
        $(
            impl StreamWriter for $t {
                fn write_to<W: std::io::Write>(&self, buffer: &mut W, order: ByteOrder) -> std::io::Result<()> {
                    let bytes = match order {
                        ByteOrder::BigEndian => {
                            self.to_be_bytes()
                        }
                        ByteOrder::LittleEndian => {
                            self.to_le_bytes()
                        }
                    };
                    buffer.write_all(&bytes)
                }
            }
        )*
    };
}

w_number![u16, u32, u64, u128, i16, i32, i64, i128, f32, f64];

impl<T: StreamWriter> StreamWriter for Vec<T> {
    fn write_to<W: std::io::Write>(&self, buffer: &mut W, order: ByteOrder) -> std::io::Result<()> {
        (self.len() as u32).write_to(buffer, order)?;
        for item in self {
            item.write_to(buffer, order)?;
        }
        Ok(())
    }
}

pub use self::{
    array::{ArrayDescription, ArrayKind},
    boolean::BooleanDescription,
    decimal::{DecimalDescription, DecimalKind},
    errors::{XError, XErrorKind},
    for_3rd::DateTime,
    integer::{IntegerDescription, IntegerKind},
    language::LanguageDescription,
    string::StringDescription,
    typing::*,
    value::{XCellValue, color::ColorDescription, time::TimeDescription},
    vector::VectorDescription,
};
pub use itertools::Itertools;
pub use validatus::Validation::{Failure, Success};

pub type XResult<T = ()> = Result<T, XError>;
pub type Validation<T> = validatus::Validation<T, XError>;

pub(crate) mod utils;

mod array;
mod boolean;
pub mod codegen;
mod custom;
mod decimal;
pub mod enumerate;
mod errors;
pub mod for_3rd;
mod integer;
mod language;
mod string;
mod typing;
mod value;
mod vector;
