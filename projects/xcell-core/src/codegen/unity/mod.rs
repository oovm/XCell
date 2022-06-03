use tera::Context;

use crate::XCellHeaders;

use super::*;

impl UnityCodegen {
    pub fn write_xml(&self, table: &XCellTable, f: &mut impl Write) -> Result<(), XError> {
        todo!()
    }
    pub fn write_csharp2(&self, table: &XCellTable, path: &Path) -> Result<(), XError> {
        let mut file = File::create(path)?;
        self.write_namespace(&mut file, include_str!("PartNamespace.cs"))?;
        self.write_cs_base(table, &mut file)?;
        if self.support_binary {
            self.write_cs_binary(table, &mut file)?;
        }
        if self.support_clone {
            self.write_cs_clone(table, &mut file)?;
        }
        file.write_all("}\n".as_bytes())?;
        Ok(())
    }
    pub fn write_class(&self, table: &XCellTable, path: &Path) -> Result<(), XError> {
        let mut file = File::create(path)?;
        let mut tera = Tera::default();
        tera.autoescape_on(vec![]);
        tera.add_raw_template("T", include_str!("PartClass.cs")).unwrap();
        let out = tera.render("T", &self.make_context(table)).unwrap();
        file.write_all(out.as_bytes())?;
        Ok(())
    }
    pub fn write_enum(&self, table: &XCellTable, path: &Path) -> Result<(), XError> {
        let mut file = File::create(path)?;
        let mut tera = Tera::default();
        tera.autoescape_on(vec![]);
        tera.add_raw_template("T", include_str!("PartEnum.cs")).unwrap();
        let out = tera.render("T", &self.make_context(table)).unwrap();
        file.write_all(out.as_bytes())?;
        Ok(())
    }
    pub fn write_interface(&self, f: &mut impl Write) -> std::io::Result<usize> {
        self.write_namespace(f, include_str!("DefineInterface.cs"))
    }
}

impl UnityCodegen {
    fn make_context(&self, table: &XCellTable) -> Context {
        let mut ctx = Context::new();
        ctx.insert("SUPPORT_BINARY", &self.support_binary);
        ctx.insert("SUPPORT_CLONE", &self.support_clone);
        ctx.insert("NAMESPACE", &self.namespace.join("."));
        ctx.insert("TABLE_NAME", &format!("{}{}", table.class_name(), self.table_suffix));
        ctx.insert("ELEMENT_NAME", &format!("{}{}", table.class_name(), self.element_suffix));
        ctx.insert("ELEMENT_GETTER", &format!("Get{}", self.element_suffix));
        ctx
    }

    fn write_namespace(&self, f: &mut impl Write, template: &str) -> std::io::Result<usize> {
        let ns = self.namespace.join(".");
        let define = template.replace("__NAMESPACE__", &ns);
        f.write(define.as_bytes())
    }
    fn write_cs_base(&self, table: &XCellTable, f: &mut impl Write) -> std::io::Result<()> {
        let indent = " ".repeat(4);
        for line in self.templated(table, include_str!("PartClass.cs")).lines() {
            writeln!(f, "{indent}{line}")?
        }
        table.headers.write_csharp(f)?;
        writeln!(f, "{indent}}}")
    }
    fn write_cs_binary(&self, table: &XCellTable, f: &mut impl Write) -> std::io::Result<()> {
        let indent = " ".repeat(4);
        for line in self.templated(table, include_str!("PartBinary1.cs")).lines() {
            writeln!(f, "{indent}{line}")?
        }
        table.headers.write_cs_br(f)?;
        for line in include_str!("PartBinary2.cs").lines() {
            writeln!(f, "{indent}{line}")?
        }
        table.headers.write_cs_bw(f)?;
        for line in include_str!("PartBinary3.cs").lines() {
            writeln!(f, "{indent}{line}")?
        }
        Ok(())
    }
    fn write_cs_clone(&self, table: &XCellTable, f: &mut impl Write) -> std::io::Result<()> {
        let indent = " ".repeat(4);
        for line in self.templated(table, include_str!("PartClone1.cs")).lines() {
            writeln!(f, "{indent}{line}")?
        }
        Ok(())
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
    fn write_csharp(&self, f: &mut impl Write) -> std::io::Result<()> {
        let indent = " ".repeat(8);
        for (idx, header) in self.iter().enumerate() {
            if idx != 0 {
                write_newline(f)?
            }
            header.write_cs_field(f, &indent)?;
        }
        Ok(())
    }
    fn write_cs_br(&self, f: &mut impl Write) -> std::io::Result<()> {
        let indent = " ".repeat(12);
        for (_, header) in self.iter().enumerate() {
            header.write_cs_br(f, &indent)?;
        }
        Ok(())
    }
    fn write_cs_bw(&self, f: &mut impl Write) -> std::io::Result<()> {
        let indent = " ".repeat(12);
        for (_, header) in self.iter().enumerate() {
            header.write_cs_bw(f, &indent)?;
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
        match &self.typing {
            XCellTyped::Boolean(v) => {
                writeln!(f, "{indent}public bool {} = {};", self.field_name, v.default)
            }
            XCellTyped::Integer8(v) => {
                writeln!(f, "{indent}public byte {} = {};", self.field_name, v.default)
            }
            XCellTyped::Integer16(v) => {
                writeln!(f, "{indent}public short {} = {};", self.field_name, v.default)
            }
            XCellTyped::Integer32(v) => {
                writeln!(f, "{indent}public int {} = {};", self.field_name, v.default)
            }
            XCellTyped::Integer64(v) => {
                writeln!(f, "{indent}public long {} = {};", self.field_name, v.default)
            }
            XCellTyped::Unsigned8(v) => {
                writeln!(f, "{indent}public sbyte {} = {};", self.field_name, v.default)
            }
            XCellTyped::Unsigned16(v) => {
                writeln!(f, "{indent}public ushort {} = {};", self.field_name, v.default)
            }
            XCellTyped::Unsigned32(v) => {
                writeln!(f, "{indent}public uint {} = {};", self.field_name, v.default)
            }
            XCellTyped::Unsigned64(v) => {
                writeln!(f, "{indent}public ulong {} = {};", self.field_name, v.default)
            }
            XCellTyped::Float32(v) => {
                writeln!(f, "{indent}public float {} = {};", self.field_name, v.default)
            }
            XCellTyped::Float64(v) => {
                writeln!(f, "{indent}public double {} = {};", self.field_name, v.default)
            }
            XCellTyped::Decimal128(_) => {
                todo!()
            }
            XCellTyped::String(v) => {
                writeln!(f, "{indent}public string {} = {:?};", self.field_name, v.default)
            }
            XCellTyped::Datetime(_) => {
                todo!()
            }
            XCellTyped::Color(_) => {
                todo!()
            }
            XCellTyped::Custom(v) => {
                writeln!(f, "{indent}public {} {};", v.typing, self.field_name)
            }
        }
    }
    /// `testBool = r.ReadBoolean();`
    fn write_cs_br(&self, f: &mut impl Write, indent: &str) -> std::io::Result<()> {
        match self.typing {
            XCellTyped::Boolean(_) => writeln!(f, "{indent}{} = r.ReadBoolean();", self.field_name),
            XCellTyped::Integer8(_) => {
                writeln!(f, "{indent}{} = r.ReadByte();", self.field_name)
            }
            XCellTyped::Integer16(_) => {
                writeln!(f, "{indent}{} = r.ReadInt16();", self.field_name)
            }
            XCellTyped::Integer32(_) => writeln!(f, "{indent}{} = r.ReadInt32();", self.field_name),
            XCellTyped::Integer64(_) => {
                writeln!(f, "{indent}{} = r.ReadUInt16();", self.field_name)
            }
            XCellTyped::Unsigned8(_) => {
                writeln!(f, "{indent}{} = r.ReadSByte();", self.field_name)
            }
            XCellTyped::Unsigned16(_) => {
                writeln!(f, "{indent}{} = r.ReadUInt16();", self.field_name)
            }
            XCellTyped::Unsigned32(_) => writeln!(f, "{indent}{} = r.ReadUInt32();", self.field_name),
            XCellTyped::Unsigned64(_) => {
                writeln!(f, "{indent}{} = r.ReadUInt64();", self.field_name)
            }
            XCellTyped::Float32(_) => {
                writeln!(f, "{indent}{} = r.ReadSingle();", self.field_name)
            }
            XCellTyped::Float64(_) => {
                writeln!(f, "{indent}{} = r.ReadDouble();", self.field_name)
            }
            XCellTyped::Decimal128(_) => {
                todo!()
            }
            XCellTyped::String(_) => writeln!(f, "{indent}{} = r.ReadString();", self.field_name),
            XCellTyped::Datetime(_) => {
                todo!()
            }
            XCellTyped::Color(_) => {
                todo!()
            }
            XCellTyped::Custom(_) => writeln!(f, "{indent}{} = r.Read();", self.field_name),
        }
    }
    /// `w.Write(testBool);`
    fn write_cs_bw(&self, f: &mut impl Write, indent: &str) -> std::io::Result<()> {
        writeln!(f, "{indent}w.Write({});", self.field_name)
    }
}

impl XCellValue {}
