use crate::XCellHeaders;

use super::*;

impl UnityCodegen {
    pub fn write_xml(&self, table: &XCellTable, f: &mut impl Write) -> Result<(), XError> {
        todo!()
    }
    pub fn write_csharp(&self, table: &XCellTable, path: &Path) -> Result<(), XError> {
        let mut file = File::create(path)?;
        self.write_namespace(&mut file, include_str!("PartNamespace.cs"))?;
        match self.namespace_legacy {
            true => file.write("\n{\n".as_bytes())?,
            false => file.write(";\n".as_bytes())?,
        };
        let indent = match self.namespace_legacy {
            true => " ".repeat(4),
            false => " ".repeat(0),
        };
        self.write_cs_base(table, &mut file, &indent)?;
        if self.support_binary {
            self.write_cs_binary(table, &mut file, &indent)?;
        }

        match self.namespace_legacy {
            true => file.write("}\n".as_bytes())?,
            false => file.write("\n".as_bytes())?,
        };
        Ok(())
    }

    pub fn write_interface(&self, f: &mut impl Write) -> std::io::Result<usize> {
        self.write_namespace(f, include_str!("DefineInterface.cs"))
    }
}

impl UnityCodegen {
    fn write_namespace(&self, f: &mut impl Write, template: &str) -> std::io::Result<usize> {
        let ns = self.namespace.join(".");
        let define = template.replace("__NAMESPACE__", &ns);
        f.write(define.as_bytes())
    }
    fn write_cs_base(&self, table: &XCellTable, f: &mut impl Write, indent: &str) -> std::io::Result<()> {
        for line in self.templated(table, include_str!("PartBase.cs")).lines() {
            writeln!(f, "{indent}{line}")?
        }
        table.headers.write_csharp(f, self.namespace_legacy)?;
        writeln!(f, "{indent}}}")
    }
    fn write_cs_binary(&self, table: &XCellTable, f: &mut impl Write, indent: &str) -> std::io::Result<()> {
        for line in self.templated(table, include_str!("PartBinary1.cs")).lines() {
            writeln!(f, "{indent}{line}")?
        }
        table.headers.write_csharp(f, self.namespace_legacy)?;
        writeln!(f, "{indent}}}")
    }
    fn templated(&self, table: &XCellTable, template: &str) -> String {
        let table_name = format!("{}{}", table.class_name(), self.table_suffix);
        let element_name = format!("{}{}", table.class_name(), self.element_suffix);
        let element_getter = format!("Get{}", self.element_suffix);
        template
            .replace("__TABLE_NAME__", &table_name)
            .replace("__ELEMENT_NAME__", &element_name)
            .replace("__ELEMENT_GETTER__", &element_getter)
    }
}

impl XCellHeaders {
    fn write_csharp(&self, f: &mut impl Write, namespace_legacy: bool) -> std::io::Result<()> {
        let indent = match namespace_legacy {
            true => " ".repeat(8),
            false => " ".repeat(4),
        };
        for (idx, header) in self.iter().enumerate() {
            if idx != 0 {
                write_newline(f)?
            }
            header.write_cs_field(f, &indent)?;
        }
        Ok(())
    }
    fn write_cs_br(&self, f: &mut impl Write, namespace_legacy: bool) -> std::io::Result<()> {
        let indent = match namespace_legacy {
            true => " ".repeat(8),
            false => " ".repeat(4),
        };
        for (idx, header) in self.iter().enumerate() {
            if idx != 0 {
                write_newline(f)?
            }
            header.write_cs_field(f, &indent)?;
        }
        Ok(())
    }
    fn write_cs_bw(&self, f: &mut impl Write, namespace_legacy: bool) -> std::io::Result<()> {
        let indent = match namespace_legacy {
            true => " ".repeat(8),
            false => " ".repeat(4),
        };
        for (idx, header) in self.iter().enumerate() {
            if idx != 0 {
                write_newline(f)?
            }
            header.write_cs_field(f, &indent)?;
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
    fn write_cs_field(&self, f: &mut impl Write, indent: &str) -> std::io::Result<()> {
        let docs = include_str!("WriteCsField.cs") //
            .replace("__SUMMARY__", &self.summary)
            .replace("__REMARKS__", &self.details);
        for line in docs.lines() {
            writeln!(f, "{indent}/// {}", line)?;
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
