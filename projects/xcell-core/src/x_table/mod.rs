use std::{
    fmt::{Debug, Formatter},
    path::{Path, PathBuf},
};

use calamine::DataType;
use serde::{Deserialize, Serialize};

use crate::{typing::XCellTyped, x_table::table_config::TableConfig, *};

pub mod global_config;
pub mod header;
pub mod table;
pub mod table_config;

pub struct XCellTable {
    pub path: PathBuf,
    pub headers: Vec<XCellHeader>,
    pub config: TableConfig,
    /// Excel 的校验和
    pub sum_excel: u64,
    /// 全局配置和本地配置的校验和
    pub sum_config: u64,
    pub errors: Vec<XError>,
}

#[derive(Debug, Clone)]
pub struct XCellHeader {
    pub column: usize,
    pub comment: String,
    pub details: String,
    pub typing: XCellTyped,
    pub field_name: String,
}
