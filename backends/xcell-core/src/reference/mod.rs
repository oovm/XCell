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
    /// 默认值（空引用为 None 或空字符串）
    pub default: Option<String>,
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

    /// 从单元格中解析引用 ID（支持整数和字符串主键）
    ///
    /// # 参数
    /// * `cell` - 要解析的单元格数据
    ///
    /// # 返回值
    /// 返回解析的结果，成功时返回 XCellValue::Reference(String)，失败时返回 XError
    pub fn parse_cell(&self, cell: &Data) -> XResult<XCellValue> {
        let value: Option<String> = match cell {
            Data::Int(v) => v.to_i64().map(|n| n.to_string()),
            Data::Float(v) => v.to_i64().map(|n| n.to_string()),
            Data::String(v) => {
                if v.trim().is_empty() {
                    None
                } else {
                    Some(v.clone())
                }
            }
            Data::Bool(v) => Some(if *v { "1" } else { "0" }.to_string()),
            Data::Empty => None,
            Data::Error(e) => return syntax_error(format!("未知错误 {e}")),
            Data::DateTime(dt) => Some(dt.to_string()),
            Data::DateTimeIso(v) => Some(v.clone()),
            Data::DurationIso(v) => Some(v.clone()),
        };
        match value {
            Some(v) => Ok(XCellValue::Reference(v)),
            None => match &self.default {
                Some(v) => Ok(XCellValue::Reference(v.clone())),
                None => Ok(XCellValue::Reference(String::new())),
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
