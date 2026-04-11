use crate::{XClassData, XDictData, XEnumerateTable, XListData};
use calamine::{Data, Rows};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::Debug,
    path::{Path, PathBuf},
};

use xcell_core::{EnumerateDescription, XCellTyped, XCellValue, for_3rd::BigInt};

use crate::{
    CalamineTable, ProjectConfig, TableConfig, WorkspaceManager, XCellHeader, XEnumerateData, XError, XResult,
};
use xcell_provider::find_first_table;

pub mod class;
pub mod dictionary;
pub mod enumerate;
pub mod header;
pub mod language;
pub mod table;

pub use table::load_table;
