use std::{
    any::type_name,
    fmt::{Debug, Display, Formatter},
};

use serde::{
    Deserialize, Serialize,
    de::{MapAccess, Visitor},
};

use crate::{
    XResult,
    for_3rd::{Data, read_map_next_extra, read_map_next_key_lowercase, read_map_next_value},
};

use crate::{BooleanDescription, LanguageDescription, XCellValue};
pub use crate::{
    array::{ArrayDescription, ArrayKind},
    decimal::DecimalDescription,
    enumerate::EnumerateDescription,
    integer::{IntegerDescription, IntegerKind},
    list::ListDescription,
    reference::ReferenceDescription,
    string::StringDescription,
    value::{color::ColorDescription, time::TimeDescription},
    vector::VectorDescription,
};

mod der;
mod display;
mod parser;
mod ser;

/// 类型元信息结构体，包含各种类型的描述信息。
#[derive(Debug, Clone, Default, Serialize)]
pub struct TypeMetaInfo {
    /// 布尔类型描述
    pub boolean: BooleanDescription,
    /// 字符串类型描述
    pub string: StringDescription,
    /// 向量类型描述
    pub vector: VectorDescription,
    /// 语言类型描述
    pub language: LanguageDescription,
    /// 枚举类型描述
    pub enumerate: EnumerateDescription,
}

/// XCell 类型枚举，支持多种数据类型。
#[derive(Clone, Serialize, Deserialize)]
pub enum XCellTyped {
    /// 布尔类型
    Boolean(Box<BooleanDescription>),
    /// 整数类型
    Integer(Box<IntegerDescription>),
    /// 小数类型
    Decimal(Box<DecimalDescription>),
    /// 字符串类型
    String(Box<StringDescription>),
    /// 时间类型
    Time(Box<TimeDescription>),
    /// 颜色类型
    Color(Box<ColorDescription>),
    /// 枚举类型
    Enumerate(Box<EnumerateDescription>),
    /// 数组类型
    Array(Box<ArrayDescription>),
    /// 向量类型
    Vector(Box<VectorDescription>),
    /// 引用类型
    Reference(Box<ReferenceDescription>),
    /// 列表类型
    List(Box<ListDescription>),
}

impl Default for XCellTyped {
    /// 返回默认的 XCellTyped 实例，默认为 Boolean 类型。
    fn default() -> Self {
        Self::Boolean(Box::default())
    }
}

impl XCellTyped {
    /// 解析单元格数据，根据类型返回对应的 XCellValue。
    ///
    /// # 参数
    /// * `cell` - 要解析的单元格数据
    ///
    /// # 返回值
    /// 返回解析的结果，成功时返回 XCellValue，失败时返回 XError。
    pub fn parse_cell(&self, cell: &Data) -> XResult<XCellValue> {
        match self {
            XCellTyped::Boolean(typing) => typing.parse_cell(cell),
            XCellTyped::Integer(typing) => typing.parse_cell(cell),
            XCellTyped::Decimal(typing) => typing.parse_cell(cell),
            XCellTyped::String(typing) => typing.parse_cell(cell),
            XCellTyped::Time(typing) => typing.parse_cell(cell),
            XCellTyped::Color(typing) => typing.parse_cell(cell),
            XCellTyped::Enumerate(typing) => typing.parse_cell(cell),
            XCellTyped::Array(typing) => typing.parse_cell(cell),
            XCellTyped::Vector(typing) => typing.parse_cell(cell),
            XCellTyped::Reference(typing) => typing.parse_cell(cell),
            XCellTyped::List(typing) => typing.parse_cell(cell),
        }
    }
}
