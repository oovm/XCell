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
    config::{ProjectConfig, TableConfig, TableLineMode},
    utils::{find_first_table, first_not_nil},
    CalamineTable, CalamineTable3, Success, WorkspaceManager, XCellHeader, XDocument, XDataItem, XEnumerateData, XError,
    XExportData, XResult, XTable,
};

pub mod comment;
pub mod data;
pub mod enumerate;
pub mod header;
pub mod language;
pub mod table;
