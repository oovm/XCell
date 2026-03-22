use serde::Serialize;

use crate::{
    XResult,
    for_3rd::{Data, ToPrimitive},
    utils::syntax_error,
    value::XCellValue,
};

mod der;

/// 引用类型描述，用于表示对其他表中行的引用
#[derive(Debug, Clone, Serialize)]
pub struct ReferenceDescription {
    /// 目标表名
    pub target_table: String,
    /// 默认值（空引用为 None 或 0）
    pub default: Option<i64>,
}

impl From<ReferenceDescription> for XCellTyped {
    fn from(value: ReferenceDescription) -> Self {
        Self::Reference(Box::new(value))
    }
}

impl ReferenceDescription {
    /// 创建新的引用类型描述
    ///
    /// # 参数
    /// * `target_table` - 目标表名
    ///
    /// # 返回值
    /// 返回新创建的 ReferenceDescription 实例
    pub fn new<S>(target_table: S) -> Self
    where
        S: Into<String>,
    {
        Self { target_table: target_table.into(), default: None }
    }

    /// 从单元格中解析引用 ID（整数或字符串）
    ///
    /// # 参数
    /// * `cell` - 要解析的单元格数据
    ///
    /// # 返回值
    /// 返回解析的结果，成功时返回 XCellValue，失败时返回 XError
    pub fn parse_cell(&self, cell: &Data) -> XResult<XCellValue> {
        let value = match cell {
            Data::Int(v) => v.to_i64(),
            Data::Float(v) => v.to_i64(),
            Data::String(v) => v.parse::<i64>().ok(),
            Data::Bool(v) => Some(if *v { 1 } else { 0 }),
            Data::Empty => self.default,
            Data::Error(e) => return syntax_error(format!("未知错误 {e}")),
            Data::DateTime(_) => None,
            Data::DateTimeIso(v) => v.parse::<i64>().ok(),
            Data::DurationIso(v) => v.parse::<i64>().ok(),
        };
        match value {
            Some(v) => Ok(XCellValue::Reference(v)),
            None => match self.default {
                Some(v) => Ok(XCellValue::Reference(v)),
                None => syntax_error(format!("无法解析引用 ID: {cell}")),
            },
        }
    }
}

impl XCellTyped {
    /// 获取引用类型描述
    ///
    /// # 返回值
    /// 如果当前类型是 Reference，返回 Some(&ReferenceDescription)，否则返回 None
    pub fn as_reference(&self) -> Option<&ReferenceDescription> {
        match self {
            XCellTyped::Reference(r) => Some(r),
            _ => None,
        }
    }

    /// 获取可变引用类型描述
    ///
    /// # 返回值
    /// 如果当前类型是 Reference，返回 Some(&mut ReferenceDescription)，否则返回 None
    pub fn mut_reference(&mut self) -> Option<&mut ReferenceDescription> {
        match self {
            XCellTyped::Reference(r) => Some(r),
            _ => None,
        }
    }

    /// 检查是否为引用类型
    ///
    /// # 返回值
    /// 如果当前类型是 Reference，返回 true，否则返回 false
    pub fn is_reference(&self) -> bool {
        self.as_reference().is_some()
    }
}

use crate::typing::XCellTyped;
