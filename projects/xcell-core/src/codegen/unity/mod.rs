use std::io::Write;

use crate::{codegen::UnityCodegen, XCellTable, XError};

impl UnityCodegen {
    pub fn write_xml(&self, table: &XCellTable, f: impl Write) -> Result<(), XError> {
        todo!()
    }
    pub fn write_csharp(&self, table: &XCellTable, f: impl Write) -> Result<(), XError> {
        todo!()
    }
    pub fn write_interface(&self, f: impl Write) {
        let t = include_str!("DefineInterface.cs");
    }
}

use new_string_template::template::Template;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    // The following regex requires at least one space between "{{" and "}}" and allows variables with spaces
    let custom_regex = Regex::new(r"(?mi)\{\{\s+([^\}]+)\s+\}\}").unwrap();
    let templ_str = "Something {{ data1 }} be {{ data2 }}, and {{ data 3 }}";
    let templ = Template::new(templ_str).with_regex(&custom_regex);
    let data = {
        let mut map = HashMap::new();
        map.insert("data1", "should");
        map.insert("data2", "here");
        map.insert("data 3", "here too");
        map
    };

    let rendered = templ.render_nofail(&data);
    assert_eq!("Something should be here, and here too", rendered);
}
