use crate::{
    x_table::dictionary::data::XDataLine, UnityCodegen, WorkspaceManager, XCellHeader, XClassData, XClassItem, XDictData,
    XEnumerateData, XListData,
};
use askama::Template;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display, Formatter},
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};
use xcell_errors::{for_3rd::Url, XError, XResult};
use xcell_types::{
    codegen::{CSharpReader, CSharpWriter},
    ByteOrder, Itertools, StreamWriter, XCellValue,
};

pub mod binary;
pub mod readable;
pub mod unity;
pub mod xml;

pub struct CsvCodegen {}
