use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
    path::{Path, PathBuf},
};

use array2d::Array2D;
use calamine::DataType;
use serde::{Deserialize, Serialize};

use xcell_errors::for_3rd::BigInt;
use xcell_types::{XCellTyped, XCellValue, XTableKind};

use crate::{
    config::TableConfig,
    utils::{find_first_table, read_table_data, read_table_headers, xx_file, xx_hash},
    XData, XError, XResult,
};

pub mod data;
pub mod header;
pub mod table;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XCellTable {
    /// 表格的绝对路径
    pub path: PathBuf,
    /// 表格的名称, 同时也是生成的类名
    pub name: String,
    /// 表格的额外配置
    pub config: TableConfig,
    /// 表格中的有效数据
    pub data: XData,
    /// Excel 的校验和
    pub sum_excel: u64,
    /// 全局配置和本地配置的校验和
    pub sum_config: u64,
}

impl XCellTable {
    pub fn is_enumerate(&self) -> bool {
        match self.data {
            XData::Enumerate(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XCellHeaders {
    pub kind: XTableKind,
    pub inner: Vec<XCellHeader>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XCellHeader {
    /// 位置
    pub column: usize,
    /// 短描述
    pub summary: String,
    /// 长描述, 鼠标悬浮时显示
    pub details: String,
    /// 类型信息
    pub typing: XCellTyped,
    /// 字段名
    pub field_name: String,
}
