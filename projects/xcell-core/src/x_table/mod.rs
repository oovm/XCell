use std::{
    fmt::{Debug, Formatter},
    path::Path,
};

use calamine::{open_workbook_auto, DataType, Reader};
use serde::{Deserialize, Serialize};

use crate::*;

pub mod header;
pub mod meta_info;
pub mod table;
pub mod typing;

#[derive(Clone)]
pub struct XCellTable {
    pub table: CalamineTable,
    pub headers: Vec<XCellHeader>,
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
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XCellMetaInfo {}
