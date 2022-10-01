use std::fmt::{Debug, Display, Formatter};

use askama::Template;
use serde::Deserialize;

use xcell_errors::XError;

use crate::XEnumerateData;

use super::*;

#[derive(Template)]
#[template(path = "BuildEnumerate.cs.djv", ext = "txt", escape = "none")]
pub struct UnityEnumerate {
    version: &'static str,
    class_name: String,
    table_name: String,
    id_type: String,
    config: UnityCodegen,
    enumerate_fields: Vec<EnumerateField>,
    value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumerateField {
    document: Vec<String>,
    remarks: Vec<String>,
    name: String,
    number: String,
    typing: String,
    getter: String,
    value: String,
}

impl Display for EnumerateField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityCodegen {
    pub fn render_enumerate(&self, table: &XEnumerateData) -> XResult<String> {
        match self.make_enumerate(table).render() {
            Ok(o) => Ok(o),
            Err(e) => Err(XError::runtime_error(e.to_string())),
        }
    }
    fn make_enumerate(&self, table: &XEnumerateData) -> UnityEnumerate {
        UnityEnumerate {
            version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            class_name: table.name.clone(),
            table_name: "".to_string(),
            id_type: "".to_string(),
            enumerate_fields: vec![],
            value: "".to_string(),
        }
    }
}
