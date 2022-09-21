use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    str::FromStr,
};

use calamine::{DataType, Rows};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use xcell_errors::{for_3rd::BigInt, Validation};
use xcell_types::{EnumerateDescription, IntegerKind, StringDescription, TypeMetaInfo, XCellTyped, XCellValue};

use crate::{
    config::{ProjectConfig, TableConfig, TableLineMode},
    utils::{find_first_table, first_not_nil},
    CalamineTable, CalamineTable3, Success, WorkspaceManager, XCellHeader, XError, XResult, XTable, XTableKind,
};

pub mod data;
pub mod enumerate;
pub mod header;
pub mod language;
pub mod table;
pub mod comment;