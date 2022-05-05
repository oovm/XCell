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
    pub table: CalamineTable,
    pub headers: Vec<XCellHeader>,
    pub config: TableConfig,
    pub errors: Vec<XError>,
}

#[derive(Debug, Clone)]
pub struct XCellHeader {
    pub column: usize,
    pub comment: String,
    pub typing: XCellTyped,
    pub field_name: String,
}
