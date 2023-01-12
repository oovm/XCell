use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

use calamine::DataType;
use serde::{Deserialize, Serialize};
use crate::config::{ProjectConfig, TableLineMode};
use xcell_errors::for_3rd::BigInt;
use xcell_types::{XCellTyped, XCellValue};
use crate::WorkspaceManager;
use crate::{
    config::TableConfig,
    utils::{find_first_table, xx_file, xx_hash},
    XError, XResult, XTableKind,
};


#[derive(Clone, Debug)]
pub struct CalamineTable {
    path: PathBuf,
    table: calamine::Range<DataType>,
    config: TableConfig,
}

pub mod data;
pub mod header;
pub mod table;
pub mod language;