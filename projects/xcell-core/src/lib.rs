#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_alias_impl_trait)]
#![feature(once_cell)]
#![feature(path_file_prefix)]
#![feature(file_create_new)]

pub use xcell_errors::{Failure, Success, Validation, XError, XErrorKind, XResult};
pub use xcell_types::*;

pub use self::{
    codegen::{binary::BinaryWriter, CsvCodegen, xml::DataContractWriter},
    config::{
        merge_rules::{MergedTable, MergeRules, MergeStep},
        PROJECT_CONFIG,
        ProjectConfig, unity::UnityCodegen, UnityBinaryConfig, WorkspaceManager,
    },
    x_table::{data::*, header::XCellHeader, language::LanguageManager, table::{CalamineTable2, XTable}},
};

mod codegen;
mod config;
pub mod utils;
mod x_table;

pub type CalamineTable = calamine::Range<calamine::DataType>;

