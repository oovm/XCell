use crate::XCellHeaders;

use super::*;

impl UnityCodegen {
    pub fn write_xml(&self, table: &XCellTable, f: &mut impl Write) -> Result<(), XError> {
        todo!()
    }
    pub fn write_csharp(&self, table: &XCellTable, path: &Path) -> Result<(), XError> {
        let mut file = File::create(path)?;
        let indent = match self.namespace_legacy {
            true => " ".repeat(4),
            false => " ".repeat(0),
        };
        self.write_cs_base(table, &mut file, &indent)?;
        Ok(())
    }
    fn write_cs_base(&self, table: &XCellTable, f: &mut impl Write, indent: &str) -> std::io::Result<()> {
        let part_table = include_str!("PartBase.cs").trim();
        let part_table = part_table
            .replace("__TABLE_NAME__", &self.table_name(table))
            .replace("__ELEMENT_NAME__", &self.element_name(table))
            .replace("__ELEMENT_GETTER__", &self.element_getter(table));
        for line in part_table.lines() {
            writeln!(f, "{indent}{line}")?
        }
        write_newline(f)?;
        let part_element = include_str!("PartElement1.cs").trim();
        let part_element = part_element.replace("__ELEMENT_NAME__", &self.element_name(table));
        for line in part_element.lines() {
            writeln!(f, "{indent}{line}")?
        }
        table.headers.write_csharp(f, self.namespace_legacy)?;
        writeln!(f, "{indent}}}")
    }

    fn table_name(&self, table: &XCellTable) -> String {
        format!("{}{}", table.class_name(), self.table_suffix)
    }
    fn element_name(&self, table: &XCellTable) -> String {
        format!("{}{}", table.class_name(), self.element_suffix)
    }
    fn element_getter(&self, _: &XCellTable) -> String {
        format!("Get{}", self.element_suffix)
    }
    pub fn write_interface(&self, f: &mut impl Write) -> std::io::Result<usize> {
        let mut slots = HashMap::new();
        slots.insert("NAMESPACE", self.namespace.join("."));
        let render = build_template(include_str!("DefineInterface.cs")).render_nofail(&slots);
        f.write(render.as_bytes())
    }
}

impl XCellHeaders {
    pub fn write_csharp(&self, f: &mut impl Write, namespace_legacy: bool) -> std::io::Result<()> {
        let indent = match namespace_legacy {
            true => " ".repeat(8),
            false => " ".repeat(4),
        };
        for (idx, header) in self.iter().enumerate() {
            if idx != 0 {
                write_newline(f)?
            }
            header.write_csharp(f, &indent)?;
        }
        Ok(())
    }
}

impl XCellHeader {
    /// ```xml
    /// /// <summary></summary>
    /// /// <remarks></remarks>
    /// [DataMember]
    /// ```
    pub fn write_csharp(&self, f: &mut impl Write, indent: &str) -> std::io::Result<()> {
        // summary
        if !self.summary.is_empty() {
            writeln!(f, "{indent}/// <summary>")?;
            for line in self.summary.lines() {
                writeln!(f, "{indent}/// {}", line)?;
            }
            writeln!(f, "{indent}/// </summary>")?;
        }
        // remarks
        if !self.details.is_empty() {
            writeln!(f, "{indent}/// <remarks>")?;
            for line in self.details.lines() {
                writeln!(f, "{indent}/// {}", line)?;
            }
            writeln!(f, "{indent}/// </remarks>")?;
        }
        writeln!(f, "{indent}[DataMember]")?;
        // fields
        write!(f, "{indent}")?;
        self.typing.write_csharp(f, &self.field_name)
    }
}

impl XCellTyped {
    pub fn write_csharp(&self, f: &mut impl Write, field: &str) -> std::io::Result<()> {
        match self {
            XCellTyped::Boolean(v) => {
                writeln!(f, "public bool {} = {};", field, v.default)
            }
            XCellTyped::Integer8(_) => {
                todo!()
            }
            XCellTyped::Integer16(_) => {
                todo!()
            }
            XCellTyped::Integer32(v) => {
                writeln!(f, "public int {} = {};", field, v.default)
            }
            XCellTyped::Integer64(_) => {
                todo!()
            }
            XCellTyped::Unsigned8(_) => {
                todo!()
            }
            XCellTyped::Unsigned16(_) => {
                todo!()
            }
            XCellTyped::Unsigned32(v) => {
                writeln!(f, "public uint {} = {};", field, v.default)
            }
            XCellTyped::Unsigned64(v) => {
                writeln!(f, "public ulong {} = {};", field, v.default)
            }
            XCellTyped::Float32(_) => {
                todo!()
            }
            XCellTyped::Float64(_) => {
                todo!()
            }
            XCellTyped::Decimal128(_) => {
                todo!()
            }
            XCellTyped::String(v) => {
                writeln!(f, "public string {} = {:?};", field, v.default)
            }
            XCellTyped::Datetime(_) => {
                todo!()
            }
            XCellTyped::Color(_) => {
                todo!()
            }
            XCellTyped::Custom(v) => {
                writeln!(f, "public {} {} = {};", v.typing, field, v.default)
            }
        }
    }
}

impl XCellValue {}
