use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    path::{Path, PathBuf},
};

use calamine::{DataType, Rows};
use serde::{Deserialize, Serialize};

use xcell_errors::{for_3rd::BigInt, Validation};
use xcell_types::{EnumerateDescription, StringDescription, TypeMetaInfo, XCellTyped, XCellValue};

use crate::{
    CalamineTable,
    CalamineTable3,
    config::{ProjectConfig, TableConfig}, Success, utils::find_first_table, WorkspaceManager, XCellHeader, XDataItem, XDocument, XEnumerateData, XError,
    XExportData, XResult, XTable,
};

pub mod class;
pub mod export;
pub mod dictionary;
pub mod enumerate;
pub mod header;
pub mod language;
pub mod table;
