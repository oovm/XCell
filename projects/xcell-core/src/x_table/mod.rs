use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    hash::Hash,
    path::{Path, PathBuf},
};

use calamine::{DataType, Rows};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use xcell_errors::for_3rd::BigInt;
use xcell_types::{EnumerateDescription, StringDescription, XCellTyped, XCellValue};

use crate::{
    config::{ProjectConfig, TableConfig},
    utils::find_first_table,
    CalamineTable, WorkspaceManager, XCellHeader, XDocument, XEnumerateData, XError, XExportData, XResult, XTable,
};

pub mod class;
pub mod dictionary;
pub mod enumerate;
pub mod export;
pub mod header;
pub mod language;
pub mod table;
