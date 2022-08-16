use std::{any::type_name, collections::BTreeSet, fmt::Formatter};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use serde_types::OneOrMany;
use xcell_errors::{
    for_3rd::{read_map_next_extra, read_map_next_value, DataType},
    XResult,
};

use crate::{XCellTyped, XCellValue};

mod der;
mod parse_cell;

#[derive(Debug, Clone, Serialize)]
pub struct VectorDescription {
    pub delimiter: BTreeSet<char>,
    pub suffix: BTreeSet<String>,
    pub typing: XCellTyped,
    pub default: Vec<XCellValue>,
}
