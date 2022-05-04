use std::{
    fmt::{Debug, Formatter},
    path::{Path, PathBuf},
};

use calamine::DataType;
use serde::{Deserialize, Serialize};

use crate::*;

pub mod header;
pub mod meta_info;
pub mod table;
pub mod typing;

pub struct XCellTable {
    pub path: PathBuf,
    pub table: CalamineTable,
    pub headers: Vec<XCellHeader>,
    pub config: XCellMetaInfo,
    pub errors: Vec<XError>,
}

#[derive(Debug, Clone)]
pub struct XCellHeader {
    pub column: usize,
    pub comment: String,
    pub typing: XCellType,
    pub field_name: String,
}

#[derive(Debug, Clone)]
pub enum XCellType {
    Boolean,
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Unsigned64,
    Float32,
    Float64,
    Float128,
    String,
    LanguageID,
    Datetime,
    Custom(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct XCellMetaInfo {
    #[serde(default, alias = "type", alias = "types")]
    pub typing: TypeMetaInfo,
}
