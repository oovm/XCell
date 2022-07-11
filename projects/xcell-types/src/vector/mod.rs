use crate::{XCellTyped, XCellValue};
use serde::{Deserialize, Serialize};
use xcell_errors::{for_3rd::DataType, XResult};

mod parse_cell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDescription {
    pub typing: XCellTyped,
    pub default: Vec<XCellValue>,
}
