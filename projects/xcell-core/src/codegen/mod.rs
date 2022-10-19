use crate::{WorkspaceManager, XClassData, XDictData, XListData};
use askama::Template;
use convert_case::{Case, Casing};
use itertools::Itertools;
use serde::Serialize;
use std::{
    fmt::{Debug, Display, Formatter},
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};
use xcell_errors::{for_3rd::Url, XResult};
use xcell_types::{
    codegen::{CSharpReader, CSharpWriter},
    ByteOrder, StreamWriter, XCellValue,
};

use serde::Deserialize;

use xcell_errors::XError;

use crate::{x_table::dictionary::data::XDataLine, UnityCodegen, XCellHeader, XEnumerateData};

pub mod binary;
pub mod readable;
pub mod unity;
pub mod xml;

pub struct CsvCodegen {}
