use fs::read_to_string;
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
    fs,
    hash::{Hash, Hasher},
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
};

use array2d::Array2D;
use calamine::DataType;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    utils::{find_first_table, read_table_data, read_table_headers, xx_file, xx_hash},
    x_table::config::{ProjectConfig, TableConfig},
    Failure, Success,
};

pub mod config;
pub mod header;
pub mod table;

#[derive(Debug, Serialize, Deserialize)]
pub struct XCellTable {
    /// 表格的绝对路径
    pub path: PathBuf,
    /// 表格的额外配置
    pub config: TableConfig,
    /// 所有需要导出的类型
    pub headers: XCellHeaders,
    /// 表格中的有效数据
    pub data: Array2D<XCellValue>,
    /// Excel 的校验和
    pub sum_excel: u64,
    /// 全局配置和本地配置的校验和
    pub sum_config: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XCellKind {
    SortedMap,
    Enumerate,
}

impl XCellTable {
    pub fn is_enumerate(&self) -> bool {
        matches!(self.headers.kind, XCellKind::Enumerate)
    }
}

impl Default for XCellKind {
    fn default() -> Self {
        Self::SortedMap
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XCellHeaders {
    pub kind: XCellKind,
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
