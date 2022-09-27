use crate::x_table::enumerate::XEnumerateData;
use askama::Template;

use super::*;

#[derive(Template)]
#[template(path = "BuildEnumerate.cs.djv")]
pub struct UnityEnumerate {
    version: &'static str,
    class_name: String,
    table_name: String,
    id_type: String,
    config: UnityCodegen,
    enumerate_fields: String,
}

impl UnityCodegen {
    pub fn render_enumerate(&self, table: &XEnumerateData) -> XResult<String> {
        Ok(self.make_enumerate(table).render()?)
    }
    fn make_enumerate(&self, table: &XEnumerateData) -> UnityEnumerate {
        UnityEnumerate {
            version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            class_name: table.name.clone(),
            table_name: table.data.key_type().as_csharp_type(),
            id_type: table.data.key_type().as_csharp_type(),
            enumerate_fields: table.data.make_enum_field(),
        }
    }
}
