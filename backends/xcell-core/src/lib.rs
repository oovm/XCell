#![warn(missing_docs)]
#![allow(clippy::get_first)]

pub use self::{
    array::{ArrayDescription, ArrayKind},
    boolean::BooleanDescription,
    decimal::{DecimalDescription, DecimalKind},
    errors::{XError, XErrorKind},
    for_3rd::DateTime,
    integer::{IntegerDescription, IntegerKind},
    language::LanguageDescription,
    list::ListDescription,
    map::MapDescription,
    optional::OptionalDescription,
    reference::ReferenceDescription,
    string::StringDescription,
    table_config::{FieldConfig, TableLineMode},
    typing::*,
    value::{XCellValue, color::ColorDescription, document::XDocument, time::TimeDescription},
    vector::VectorDescription,
};
pub use itertools::Itertools;
pub use validatus::Validation::{Failure, Success};

pub type XResult<T = ()> = Result<T, XError>;
pub type Validation<T> = validatus::Validation<T, XError>;

pub(crate) mod utils;

mod array;
mod boolean;
mod custom;
mod decimal;
pub mod enumerate;
mod errors;
pub mod for_3rd;
mod integer;
mod language;
mod list;
mod map;
mod optional;
mod reference;
mod string;
mod table_config;
mod typing;
mod value;
mod vector;
