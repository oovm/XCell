use crate::{
    errors::{syntax_error, type_mismatch},
    XCellValue,
};
use serde::{Deserialize, Serialize};
use xcell_errors::{for_3rd::DataType, XResult};

mod parse_cell;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VectorDescription {
    pub default: Vec<XCellValue>,
}
