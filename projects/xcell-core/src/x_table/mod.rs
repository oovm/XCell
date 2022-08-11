use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

use calamine::DataType;
use serde::{Deserialize, Serialize};

use xcell_errors::for_3rd::BigInt;
use xcell_types::{XCellTyped, XCellValue};

use crate::{
    config::TableConfig,
    utils::{find_first_table, xx_file, xx_hash},
    XData, XError, XResult,
};

pub mod data;
pub mod header;
pub mod table;
