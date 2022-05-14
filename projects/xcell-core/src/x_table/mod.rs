use fs::read_to_string;
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    str::FromStr,
};

use calamine::DataType;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    typing::{XCellTyped, XCellValue},
    utils::{find_first_table, read_table_data, read_table_headers, xx_hash},
    x_table::table_config::{ProjectConfig, TableConfig},
    Failure, Success, Validation, XError, XResult,
};

pub mod header;
pub mod table;
pub mod table_config;

#[derive(Debug, Serialize, Deserialize)]
pub struct XCellTable {
    /// 表格的绝对路径
    pub path: PathBuf,
    /// 表格的额外配置
    pub config: TableConfig,
    /// 所有需要导出的类型
    pub headers: Vec<XCellHeader>,
    /// 表格中的有效数据
    pub data: Vec<Vec<XCellValue>>,
    /// Excel 的校验和
    pub sum_excel: u64,
    /// 全局配置和本地配置的校验和
    pub sum_config: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XCellHeader {
    /// 位置
    pub column: usize,
    /// 短描述
    pub comment: String,
    /// 长描述, 鼠标悬浮时显示
    pub details: String,
    /// 类型信息
    pub typing: XCellTyped,
    /// 字段名
    pub field_name: String,
}
