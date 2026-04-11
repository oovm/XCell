#![warn(missing_docs)]
#![feature(try_blocks)]
#![feature(type_alias_impl_trait)]
#![feature(path_file_prefix)]

pub use xcell_core::XDocument;
pub use xcell_core::{XError, XErrorKind};
pub type XResult<T> = Result<T, XError>;

pub use xcell_config::{
    CocosCodegen, MergeRules, MergeStep, PROJECT_CONFIG, ProjectConfig, TableConfig, TableLineMode, UnityBinaryConfig,
    UnityCodegen,
};

pub use self::{
    config::WorkspaceStatus,
    config::WorkspaceManager,
    x_table::{
        class::{XClassData, XClassItem, XClassTable},
        dictionary::{
            XDictTable, XListTable,
            data::{XDataLine, XDictData, XListData},
        },
        enumerate::{DefineManager, XEnumerateTable, data::XEnumerateData},
        language::{LanguageManager, XLanguageData, XLanguageID, XLanguageTable},
        table::CalamineTable,
    },
};
pub use xcell_provider::XCellHeader;
mod config;
pub mod utils;
pub mod validation;
mod x_table;
